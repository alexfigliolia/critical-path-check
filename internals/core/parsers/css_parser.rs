use std::collections::VecDeque;
use std::path::PathBuf;

use futures::executor::block_on;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

use crate::logger::logger::Logger;
use crate::parsers::file_paths::{FilePaths, FileResolutionStrategy};
use crate::parsers::traverser::{CriticalPath, Traverser};

pub struct CSSParser {
    builder: CriticalPath,
    resolution_roots: Vec<PathBuf>,
}

impl CSSParser {
    pub fn new(
        root_directory: &PathBuf,
        paths: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> Self {
        let builder = CSSParser::create(root_directory, paths);
        CSSParser {
            resolution_roots: [builder.root_directory.to_owned()].to_vec(),
            builder,
        }
    }

    fn to_source_file(
        &self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Option<String> {
        match file {
            FileResolutionStrategy::Local(path) => {
                let path_string = &FilePaths::to_string(path);
                if let Some(source_file) = block_on(FilePaths::read_resource(path)) {
                    return Some(source_file);
                }
                Logger::path_error(path_string);
                FilePaths::store_unresolved_path(origin, path_string);
                None
            }
            FileResolutionStrategy::Http(url) => {
                if let Some(source_file) = FilePaths::fetch_resource_sync(url) {
                    return Some(source_file);
                }
                Logger::failed_to_load_file(url);
                FilePaths::store_unresolved_path(origin, url);
                None
            }
        }
    }
}

impl Traverser for CSSParser {
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
        if let Some(source_file) = self.to_source_file(&file, origin) {
            if let Ok(stylesheet) = StyleSheet::parse(&source_file, ParserOptions::default()) {
                self.builder.weight += source_file.len();
                for rule in stylesheet.rules.0 {
                    if let CssRule::Import(reference) = rule {
                        match file {
                            FileResolutionStrategy::Http(_) => {
                                let file_system = FilePaths::new(&self.builder.root_directory);
                                if let Some(strategy) =
                                    file_system.resolve_file(&reference.url, &Vec::new())
                                {
                                    self.builder.stack.push_back((strategy, file.clone()));
                                } else {
                                    self.import_reference_error(&file, &reference.url);
                                }
                            }
                            FileResolutionStrategy::Local(ref path) => {
                                let file_system = FilePaths::new(path);
                                if let Some(strategy) =
                                    file_system.resolve_file(&reference.url, &self.resolution_roots)
                                {
                                    self.builder.stack.push_back((strategy, file.clone()));
                                } else {
                                    self.import_reference_error(&file, &reference.url);
                                }
                            }
                        }
                    }
                }
            } else {
                Logger::failed_to_parse_file(&FilePaths::hash(&file));
            }
        }
    }
}
