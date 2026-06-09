use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex as StdMutex},
};

use dashmap::DashSet;
use regex::Regex;
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    logger::logger::Logger,
    visitor::{
        directory_scope::DirectoryScope, explored_paths::ExploredPaths,
        file_resolution_strategy::FileResolutionStrategy, visitor::Visitor,
    },
};

#[derive(Debug)]
pub struct SearchState {
    pub weight: usize,
    pub parser: Regex,
    pub scope: DirectoryScope,
    pub visited: DashSet<String>,
    pub paths: Arc<StdMutex<ExploredPaths>>,
}

impl SearchState {
    pub fn new(
        root: &FileResolutionStrategy,
        parser: Regex,
        paths: Arc<StdMutex<ExploredPaths>>,
    ) -> Self {
        Self {
            paths,
            parser,
            weight: 0,
            visited: DashSet::new(),
            scope: DirectoryScope::new(root),
        }
    }

    pub fn thread_safe(
        root: &FileResolutionStrategy,
        parser: Regex,
        paths: Arc<StdMutex<ExploredPaths>>,
    ) -> Arc<Mutex<SearchState>> {
        Arc::new(Mutex::new(SearchState::new(root, parser, paths)))
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
        let key: String = file.to_string();
        if !self.visited.insert(key) {
            return None;
        }
        Some(
            self.create_visitor(file)
                .with_references(self.capture_file_and_import_references(file, origin)),
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

    fn capture_file_and_import_references(
        &mut self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Vec<String> {
        let mut references: Vec<String> = Vec::new();
        match &file {
            FileResolutionStrategy::Local(path) => {
                let path_str = file.to_string();
                if let Ok(file_interface) = File::open(path)
                    && let Ok(meta) = file_interface.metadata()
                {
                    self.capture_resolved_path(origin, &path_str);
                    self.weight += meta.len() as usize;
                    let buffer = BufReader::new(file_interface);
                    for line in buffer.lines().map_while(Result::ok) {
                        references.append(&mut self.capture_regex_matches(&line));
                    }
                } else {
                    self.capture_unresolved_path(origin, &path_str);
                    Logger::path_error(&path_str);
                }
            }
            FileResolutionStrategy::Http(url) => {
                if let Some(content) = FileResolutionStrategy::fetch_resource(url) {
                    self.capture_resolved_path(origin, url);
                    self.weight += content.len();
                    references.append(&mut self.capture_regex_matches(&content));
                } else {
                    self.capture_unresolved_path(origin, url);
                    Logger::failed_to_load_file(url);
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

    fn capture_unresolved_path(&mut self, origin: &FileResolutionStrategy, path: &str) {
        self.paths
            .lock()
            .unwrap()
            .store_unresolved_path(origin, path);
    }

    fn capture_resolved_path(&mut self, origin: &FileResolutionStrategy, path: &str) {
        self.paths.lock().unwrap().store_resolved_path(origin, path);
    }
}
