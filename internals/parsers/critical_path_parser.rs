use std::sync::Arc;

use async_recursion::async_recursion;
use regex::Regex;
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    logger::logger::Logger,
    parsers::file_paths::{FilePaths, FileResolutionStrategy},
    visitor::search_state::SearchState,
};

#[derive(Debug)]
pub struct CriticalPathParser {
    pub state: Arc<Mutex<SearchState>>,
    pub stack: Vec<(FileResolutionStrategy, FileResolutionStrategy)>,
}

impl CriticalPathParser {
    pub fn new(
        root: &FileResolutionStrategy,
        regex: Regex,
        stack: Vec<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> Self {
        Self {
            stack,
            state: SearchState::thread_safe(root, regex),
        }
    }

    pub async fn analyze(&mut self) -> usize {
        let data = self.state.clone();
        let mut task_pool = JoinSet::<()>::new();
        while !self.stack.is_empty() {
            if let Some((file, origin)) = self.stack.pop() {
                let mutex = self.state.clone();
                task_pool.spawn(CriticalPathParser::dfs(file, origin, mutex));
            }
        }
        task_pool.join_all().await;
        SearchState::read(&data, |v| v.weight).await
    }

    #[async_recursion]
    async fn dfs(
        file: FileResolutionStrategy,
        origin: FileResolutionStrategy,
        state: Arc<Mutex<SearchState>>,
    ) {
        if let Some(visitor) =
            SearchState::read(&state, |mut v| v.proceed_with_search(&file, &origin)).await
        {
            let mut task_pool = JoinSet::<()>::new();
            for reference in &visitor.references {
                if let Some(strategy) = visitor.resolve_reference(reference) {
                    let origin_clone = file.clone();
                    let state_clone = state.clone();
                    task_pool.spawn(CriticalPathParser::dfs(strategy, origin_clone, state_clone));
                } else {
                    FilePaths::store_unresolved_path(&file, reference);
                    Logger::path_error(reference);
                }
            }
            task_pool.join_all().await;
        }
    }
}
