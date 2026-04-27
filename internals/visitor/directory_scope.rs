use std::path::PathBuf;

use crate::parsers::file_paths::FileResolutionStrategy;

#[derive(Clone, Debug)]
pub struct DirectoryScope {
    pub root_directory: FileResolutionStrategy,
    pub resolution_roots: Vec<PathBuf>,
}

impl DirectoryScope {
    pub fn new(root: &FileResolutionStrategy) -> Self {
        let root_directory = root.to_owned();
        let mut resolution_roots: Vec<PathBuf> = Vec::new();
        if let FileResolutionStrategy::Local(path) = root {
            resolution_roots.push(path.to_owned());
        }
        Self {
            root_directory,
            resolution_roots,
        }
    }
}
