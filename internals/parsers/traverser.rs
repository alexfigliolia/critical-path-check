use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

pub struct GraphBuilder {
    pub weight: usize,
    pub stack: VecDeque<PathBuf>,
    pub visited: HashSet<PathBuf>,
}

pub trait Traverser {
    fn create(paths: &HashSet<PathBuf>) -> GraphBuilder {
        GraphBuilder {
            weight: 0,
            visited: HashSet::new(),
            stack: VecDeque::from_iter(paths.clone()),
        }
    }

    fn traverse(&mut self) -> usize;

    fn dfs(&mut self, path: PathBuf);
}
