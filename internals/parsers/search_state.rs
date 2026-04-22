use std::{path::PathBuf, sync::Arc};

use dashmap::DashSet;
use regex::Regex;
use tokio::sync::{Mutex, MutexGuard};

use crate::parsers::file_paths::FileResolutionStrategy;

#[derive(Clone)]
pub struct DirectoryScope {
    pub root_directory: FileResolutionStrategy,
    pub resolution_roots: Vec<PathBuf>,
}

#[derive(Clone)]
pub struct ParsingConfiguration {
    pub regex: Regex,
    pub capture_position: usize,
}

pub struct SearchState {
    pub weight: usize,
    pub scope: DirectoryScope,
    pub parser: ParsingConfiguration,
    pub visited: DashSet<String>,
}

impl SearchState {
    pub fn new(parser: ParsingConfiguration, scope: DirectoryScope) -> Self {
        Self {
            scope,
            parser,
            weight: 0,
            visited: DashSet::new(),
        }
    }

    pub async fn read<R>(
        state: &Arc<Mutex<SearchState>>,
        writer: impl FnOnce(MutexGuard<SearchState>) -> R,
    ) -> R {
        let current_state = state.lock().await;
        writer(current_state)
    }
}
