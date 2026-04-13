use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use crate::parsers::file_paths::FileResolutionStrategy;

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
}
