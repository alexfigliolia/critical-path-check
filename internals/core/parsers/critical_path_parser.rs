use std::sync::{Arc, Mutex as StdMutex};

use async_recursion::async_recursion;
use regex::Regex;
use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    logger::logger::Logger,
    visitor::{
        explored_paths::ExploredPaths, file_resolution_strategy::FileResolutionStrategy,
        search_state::SearchState,
    },
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
        paths: Arc<StdMutex<ExploredPaths>>,
        stack: Vec<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> Self {
        Self {
            stack,
            state: SearchState::thread_safe(root, regex, paths),
        }
    }

    pub async fn analyze(&mut self) -> usize {
        let data = self.state.clone();
        let mut task_pool = JoinSet::<()>::new();
        while !self.stack.is_empty() {
            if let Some((file, origin)) = self.stack.pop() {
                let state = self.state.clone();
                task_pool.spawn(CriticalPathParser::dfs(file, origin, state));
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
                    SearchState::read(&state, |v| {
                        v.paths
                            .lock()
                            .unwrap()
                            .store_unresolved_path(&file, reference)
                    })
                    .await;
                    Logger::path_error(reference);
                }
            }
            task_pool.join_all().await;
        }
    }
}
