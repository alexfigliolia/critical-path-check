use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};

use async_recursion::async_recursion;
use regex::Regex;
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    logger::logger::Logger,
    parsers::{
        file_paths::{FilePaths, FileResolutionStrategy},
        search_state::{DirectoryScope, ParsingConfiguration, SearchState},
    },
};

pub struct CriticalPathParser {
    pub stack: Vec<(FileResolutionStrategy, FileResolutionStrategy)>,
    pub state: Arc<Mutex<SearchState>>,
}

impl CriticalPathParser {
    pub fn new(
        root_directory: &FileResolutionStrategy,
        stack: Vec<(FileResolutionStrategy, FileResolutionStrategy)>,
        regex: Regex,
        capture_position: usize,
    ) -> Self {
        let mut resolution_roots: Vec<PathBuf> = Vec::new();
        if let FileResolutionStrategy::Local(path) = root_directory {
            resolution_roots.push(path.to_owned());
        }
        Self {
            stack,
            state: Arc::new(Mutex::new(SearchState::new(
                ParsingConfiguration {
                    regex,
                    capture_position,
                },
                DirectoryScope {
                    resolution_roots,
                    root_directory: root_directory.to_owned(),
                },
            ))),
        }
    }

    pub async fn analyze(&mut self) -> usize {
        let data = self.state.clone();
        let mut task_pool = JoinSet::<()>::new();
        while !self.stack.is_empty() {
            if let Some((file, origin)) = self.stack.pop() {
                let mutex = self.state.clone();
                task_pool.spawn(CriticalPathParser::dfs(file, origin, mutex));
            }
        }
        task_pool.join_all().await;
        data.lock().await.weight
    }

    #[async_recursion]
    async fn dfs(
        file: FileResolutionStrategy,
        origin: FileResolutionStrategy,
        state: Arc<Mutex<SearchState>>,
    ) {
        let key: String = FilePaths::hash(&file);
        if !SearchState::read(&state, |v| v.visited.insert(key)).await {
            return;
        }
        let parse_results = CriticalPathParser::detect_import_paths(
            &SearchState::read(&state, |v| v.parser.clone()).await,
            &file,
            &origin,
        );
        if let Some((bytes, import_paths)) = parse_results {
            let directory_scope = SearchState::read(&state, |mut v| {
                v.weight += bytes;
                v.scope.clone()
            })
            .await;
            let mut task_pool = JoinSet::<()>::new();
            for reference in import_paths {
                match file {
                    FileResolutionStrategy::Http(_) => {
                        let file_system = FilePaths::new(&directory_scope.root_directory);
                        if let Some(strategy) = file_system.resolve_file(&reference, &Vec::new()) {
                            let origin_clone = file.clone();
                            let state_clone = state.clone();
                            task_pool.spawn(CriticalPathParser::dfs(
                                strategy,
                                origin_clone,
                                state_clone,
                            ));
                        } else {
                            CriticalPathParser::import_reference_error(&file, &reference);
                        }
                    }
                    FileResolutionStrategy::Local(ref path) => {
                        let root = path.parent().unwrap_or(path).to_path_buf();
                        let file_system = FilePaths::new(&FileResolutionStrategy::Local(root));
                        if let Some(strategy) =
                            file_system.resolve_file(&reference, &directory_scope.resolution_roots)
                        {
                            let origin_clone = file.clone();
                            let state_clone = state.clone();
                            task_pool.spawn(CriticalPathParser::dfs(
                                strategy,
                                origin_clone,
                                state_clone,
                            ));
                        } else {
                            CriticalPathParser::import_reference_error(&file, &reference);
                        }
                    }
                }
            }
            task_pool.join_all().await;
        }
    }

    fn import_reference_error(origin: &FileResolutionStrategy, reference: &str) {
        FilePaths::store_unresolved_path(origin, reference);
        Logger::path_error(reference);
    }

    fn detect_import_paths(
        parser: &ParsingConfiguration,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Option<(usize, Vec<String>)> {
        match &file {
            FileResolutionStrategy::Local(path) => {
                if let Ok(file_interface) = File::open(path)
                    && let Ok(meta) = file_interface.metadata()
                {
                    let mut referenced_files: Vec<String> = Vec::new();
                    let bytes = meta.len() as usize;
                    let buffer = BufReader::new(file_interface);
                    for line in buffer.lines().map_while(Result::ok) {
                        referenced_files.append(&mut CriticalPathParser::capture_regex_matches(
                            parser, &line,
                        ));
                    }
                    return Some((bytes, referenced_files));
                }
                FilePaths::store_unresolved_path(origin, &FilePaths::hash(file));
                Logger::failed_to_parse_file(&FilePaths::to_string(path));
                None
            }
            FileResolutionStrategy::Http(url) => {
                if let Some(content) = FilePaths::fetch_resource(url) {
                    let bytes = content.len();
                    return Some((
                        bytes,
                        CriticalPathParser::capture_regex_matches(parser, &content),
                    ));
                }
                FilePaths::store_unresolved_path(origin, url);
                Logger::failed_to_load_file(url);
                None
            }
        }
    }

    fn capture_regex_matches(parser: &ParsingConfiguration, content: &str) -> Vec<String> {
        let mut referenced_files: Vec<String> = Vec::new();
        let matches = parser.regex.captures_iter(content);
        for capture in matches {
            referenced_files.push(capture[parser.capture_position].to_owned());
        }
        referenced_files
    }
}
