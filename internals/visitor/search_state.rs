use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use dashmap::DashSet;
use regex::Regex;
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    logger::logger::Logger,
    parsers::file_paths::{FilePaths, FileResolutionStrategy},
    visitor::{directory_scope::DirectoryScope, visitor::Visitor},
};

#[derive(Debug)]
pub struct SearchState {
    pub weight: usize,
    pub parser: Regex,
    pub scope: DirectoryScope,
    pub visited: DashSet<String>,
}

impl SearchState {
    pub fn new(root: &FileResolutionStrategy, parser: Regex) -> Self {
        Self {
            parser,
            weight: 0,
            visited: DashSet::new(),
            scope: DirectoryScope::new(root),
        }
    }

    pub fn thread_safe(root: &FileResolutionStrategy, parser: Regex) -> Arc<Mutex<SearchState>> {
        Arc::new(Mutex::new(SearchState::new(root, parser)))
    }

    pub async fn read<R>(
        state: &Arc<Mutex<SearchState>>,
        writer: impl FnOnce(MutexGuard<SearchState>) -> R,
    ) -> R {
        let current_state = state.lock().await;
        writer(current_state)
    }

    pub fn proceed_with_search(
        &mut self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Option<Visitor> {
        let key: String = FilePaths::hash(file);
        if !self.visited.insert(key) {
            return None;
        }
        Some(
            self.create_visitor(file)
                .with_references(self.capture_import_references(file, origin)),
        )
    }

    fn create_visitor(&self, file: &FileResolutionStrategy) -> Visitor {
        match file {
            FileResolutionStrategy::Http(_) => Visitor::from(&self.scope.root_directory),
            FileResolutionStrategy::Local(path) => {
                Visitor::from_path(path.parent().unwrap_or(path).to_path_buf())
                    .with_possible_roots(&self.scope.resolution_roots)
            }
        }
    }

    fn capture_import_references(
        &mut self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Vec<String> {
        let mut references: Vec<String> = Vec::new();
        match &file {
            FileResolutionStrategy::Local(path) => {
                if let Ok(file_interface) = File::open(path)
                    && let Ok(meta) = file_interface.metadata()
                {
                    self.weight += meta.len() as usize;
                    let buffer = BufReader::new(file_interface);
                    for line in buffer.lines().filter_map(Result::ok) {
                        references.append(&mut self.capture_regex_matches(&line));
                    }
                } else {
                    self.capture_unresolved_path(origin, &FilePaths::hash(file));
                }
            }
            FileResolutionStrategy::Http(url) => {
                if let Some(content) = FilePaths::fetch_resource(url) {
                    self.weight += content.len();
                    references.append(&mut self.capture_regex_matches(&content));
                } else {
                    self.capture_unresolved_path(origin, url);
                }
            }
        }
        references
    }

    fn capture_regex_matches(&self, content: &str) -> Vec<String> {
        let mut references: Vec<String> = Vec::new();
        let matches = self.parser.captures_iter(content);
        for capture in matches {
            references.push(capture[1].to_owned());
        }
        references
    }

    fn capture_unresolved_path(&self, origin: &FileResolutionStrategy, path: &str) {
        FilePaths::store_unresolved_path(origin, path);
        Logger::failed_to_parse_file(path);
    }
}
