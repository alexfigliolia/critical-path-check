use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use crate::{
    logger::logger::Logger,
    parsers::file_paths::{FilePaths, FileResolutionStrategy},
};

pub struct CriticalPath {
    pub weight: usize,
    pub root_directory: PathBuf,
    pub visited: HashSet<String>,
    pub stack: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
}

pub trait Traverser {
    fn create(
        root_directory: &PathBuf,
        stack: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> CriticalPath {
        CriticalPath {
            stack,
            weight: 0,
            visited: HashSet::new(),
            root_directory: root_directory.to_owned(),
        }
    }

    fn traverse(&mut self) -> usize;

    fn dfs(&mut self, file: FileResolutionStrategy, origin: &FileResolutionStrategy);

    fn import_reference_error(&self, origin: &FileResolutionStrategy, reference: &str) {
        FilePaths::store_unresolved_path(origin, reference);
        Logger::path_error(reference);
    }
}
