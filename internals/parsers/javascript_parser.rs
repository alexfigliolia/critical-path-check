use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::LazyLock,
};

use regex::Regex;

use crate::{
    logger::logger::Logger,
    parsers::{
        file_paths::{FilePaths, FileResolutionStrategy},
        traverser::{CriticalPath, Traverser},
    },
};

static IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(import\s?|from\s?)['"]([^'"]+)['"]"#).unwrap());

pub struct JavaScriptParser {
    builder: CriticalPath,
    resolution_roots: Vec<PathBuf>,
}

impl JavaScriptParser {
    pub fn new(
        root_directory: &PathBuf,
        paths: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> Self {
        let builder = JavaScriptParser::create(root_directory, paths);
        JavaScriptParser {
            resolution_roots: [builder.root_directory.to_owned()].to_vec(),
            builder,
        }
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
        let matches = IMPORT_REGEX.captures_iter(content);
        for capture in matches {
            referenced_files.push(capture[2].to_owned());
        }
        referenced_files
    }
}

impl Traverser for JavaScriptParser {
    fn traverse(&mut self) -> usize {
        while !self.builder.stack.is_empty() {
            if let Some((file, origin)) = self.builder.stack.pop_back() {
                self.dfs(file, &origin);
            }
        }
        self.builder.weight
    }

    fn dfs(&mut self, file: FileResolutionStrategy, origin: &FileResolutionStrategy) {
        let key = FilePaths::hash(&file);
        if self.builder.visited.contains(&key) {
            return;
        }
        self.builder.visited.insert(key);
        if let Some((bytes, import_paths)) = self.detect_import_paths(&file, origin) {
            self.builder.weight += bytes;
            for reference in import_paths {
                match file {
                    FileResolutionStrategy::Http(_) => {
                        let file_system = FilePaths::new(&self.builder.root_directory);
                        if let Some(strategy) = file_system.resolve_file(&reference, &Vec::new()) {
                            self.builder.stack.push_back((strategy, file.clone()));
                        } else {
                            self.import_reference_error(&file, &reference);
                        }
                    }
                    FileResolutionStrategy::Local(ref path) => {
                        let root = path.parent().unwrap_or(path).to_path_buf();
                        let file_system = FilePaths::new(&root);
                        if let Some(strategy) =
                            file_system.resolve_file(&reference, &self.resolution_roots)
                        {
                            self.builder.stack.push_back((strategy, file.clone()));
                        } else {
                            self.import_reference_error(&file, &reference);
                        }
                    }
                }
            }
        }
    }
}
