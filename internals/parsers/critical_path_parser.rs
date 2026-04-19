use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use regex::Regex;

use crate::{
    logger::logger::Logger,
    parsers::file_paths::{FilePaths, FileResolutionStrategy},
};

struct ParsingConfiguration {
    pub regex: Regex,
    pub capture_position: usize,
}

struct DirectoryScope {
    pub root_directory: PathBuf,
    pub resolution_roots: Vec<PathBuf>,
}

pub struct CriticalPathParser {
    pub weight: usize,
    pub visited: HashSet<String>,
    pub stack: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
    parser: ParsingConfiguration,
    scope: DirectoryScope,
}

impl CriticalPathParser {
    pub fn new(
        root_directory: &PathBuf,
        stack: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
        regex: Regex,
        capture_position: usize,
    ) -> Self {
        CriticalPathParser {
            stack,
            weight: 0,
            visited: HashSet::new(),
            parser: ParsingConfiguration {
                regex,
                capture_position,
            },
            scope: DirectoryScope {
                root_directory: root_directory.to_owned(),
                resolution_roots: [root_directory.to_owned()].to_vec(),
            },
        }
    }

    pub fn analyze(&mut self) -> usize {
        while !self.stack.is_empty() {
            if let Some((file, origin)) = self.stack.pop_back() {
                self.dfs(file, &origin);
            }
        }
        self.weight
    }

    fn dfs(&mut self, file: FileResolutionStrategy, origin: &FileResolutionStrategy) {
        let key = FilePaths::hash(&file);
        if self.visited.contains(&key) {
            return;
        }
        self.visited.insert(key);
        if let Some((bytes, import_paths)) = self.detect_import_paths(&file, origin) {
            self.weight += bytes;
            for reference in import_paths {
                match file {
                    FileResolutionStrategy::Http(_) => {
                        let file_system = FilePaths::new(&self.scope.root_directory);
                        if let Some(strategy) = file_system.resolve_file(&reference, &Vec::new()) {
                            self.stack.push_back((strategy, file.clone()));
                        } else {
                            self.import_reference_error(&file, &reference);
                        }
                    }
                    FileResolutionStrategy::Local(ref path) => {
                        let root = path.parent().unwrap_or(path).to_path_buf();
                        let file_system = FilePaths::new(&root);
                        if let Some(strategy) =
                            file_system.resolve_file(&reference, &self.scope.resolution_roots)
                        {
                            self.stack.push_back((strategy, file.clone()));
                        } else {
                            self.import_reference_error(&file, &reference);
                        }
                    }
                }
            }
        }
    }

    fn import_reference_error(&self, origin: &FileResolutionStrategy, reference: &str) {
        FilePaths::store_unresolved_path(origin, reference);
        Logger::path_error(reference);
    }

    fn detect_import_paths(
        &self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Option<(usize, Vec<String>)> {
        match file {
            FileResolutionStrategy::Local(path) => {
                if let Ok(file_interface) = File::open(path)
                    && let Ok(meta) = file_interface.metadata()
                {
                    let mut referenced_files: Vec<String> = Vec::new();
                    let bytes = meta.len() as usize;
                    let buffer = BufReader::new(file_interface);
                    for line in buffer.lines().flatten() {
                        referenced_files.append(&mut self.capture_regex_matches(&line));
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
                    return Some((bytes, self.capture_regex_matches(&content)));
                }
                FilePaths::store_unresolved_path(origin, url);
                Logger::failed_to_load_file(url);
                None
            }
        }
    }

    fn capture_regex_matches(&self, content: &str) -> Vec<String> {
        let mut referenced_files: Vec<String> = Vec::new();
        let matches = self.parser.regex.captures_iter(content);
        for capture in matches {
            referenced_files.push(capture[self.parser.capture_position].to_owned());
        }
        referenced_files
    }
}
