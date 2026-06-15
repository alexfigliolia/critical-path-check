use colored::Colorize;
use std::collections::{HashMap, HashSet};

use crate::{
    logger::logger::Logger,
    visitor::{file_resolution_strategy::FileResolutionStrategy, path_store::PathStore},
};

#[derive(Clone, Debug)]
pub struct ExploredPaths {
    pub resolved: HashMap<String, HashSet<String>>,
    pub unresolved: HashMap<String, HashSet<String>>,
}

impl ExploredPaths {
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            unresolved: HashMap::new(),
        }
    }

    pub fn store_resolved_path(&mut self, root: &FileResolutionStrategy, path: &str) {
        self.store_path(PathStore::Resolved, root, path);
    }

    pub fn store_unresolved_path(&mut self, root: &FileResolutionStrategy, path: &str) {
        self.store_path(PathStore::Unresolved, root, path);
    }

    pub fn clear_resolved_paths(&mut self) {
        self.clear_paths(PathStore::Resolved);
    }

    pub fn clear_unresolved_paths(&mut self) {
        self.clear_paths(PathStore::Unresolved);
    }

    fn store_path(&mut self, storage: PathStore, root: &FileResolutionStrategy, path: &str) {
        let store = storage.resolve_mut(self);
        let root_hash = root.to_string();
        let mut default_bucket = HashSet::new();
        let bucket = store.get_mut(&root_hash).unwrap_or(&mut default_bucket);
        bucket.insert(path.to_owned());
        let thing: HashSet<String> = bucket.clone();
        store.insert(root_hash, thing);
    }

    fn clear_paths(&mut self, storage: PathStore) {
        let store = storage.resolve_mut(self);
        *store = HashMap::new();
    }

    pub fn log_resolved(&self) {
        self.log_paths(PathStore::Resolved, None);
    }

    pub fn log_unresolved(&self) {
        self.log_paths(
            PathStore::Unresolved,
            Some(
                "The following file references were not resolved and will be omitted from analysis",
            ),
        );
    }

    fn log_paths(&self, storage: PathStore, log: Option<&str>) {
        let unresolved = storage.resolve(self);
        if !unresolved.is_empty() {
            if let Some(message) = log {
                eprintln!();
                Logger::info(message);
            }
            for (root, bucket) in unresolved.iter() {
                eprintln!(
                    "\n{}{}{}",
                    Logger::indent(None),
                    "Origin: ".cyan(),
                    root.bright_blue()
                );
                for path in bucket {
                    eprintln!(
                        "{}{}{}",
                        Logger::indent(Some(6)),
                        "References: ".cyan(),
                        path.bright_blue()
                    )
                }
            }
            eprintln!();
        }
    }
}
