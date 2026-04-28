use std::path::PathBuf;

use crate::parsers::file_paths::{FilePaths, FileResolutionStrategy};

pub struct Visitor {
    pub references: Vec<String>,
    pub resolver: FilePaths,
    pub possible_roots: Vec<PathBuf>,
}

impl Visitor {
    pub fn new(resolver: FilePaths) -> Self {
        Self {
            resolver,
            references: Vec::new(),
            possible_roots: Vec::new(),
        }
    }

    pub fn from(root: &FileResolutionStrategy) -> Self {
        Visitor::new(FilePaths::new(root))
    }

    pub fn from_path(root: PathBuf) -> Self {
        Visitor::new(FilePaths::from_owned(FileResolutionStrategy::Local(root)))
    }

    pub fn with_references(mut self, references: Vec<String>) -> Self {
        self.references = references;
        self
    }

    pub fn with_possible_roots(mut self, roots: &Vec<PathBuf>) -> Self {
        self.possible_roots = roots.to_owned();
        self
    }

    pub fn resolve_reference(&self, reference: &str) -> Option<FileResolutionStrategy> {
        self.resolver.resolve_file(reference, &self.possible_roots)
    }
}
