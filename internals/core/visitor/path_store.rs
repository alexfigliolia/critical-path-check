use std::collections::{HashMap, HashSet};

use crate::visitor::explored_paths::ExploredPaths;

pub enum PathStore {
    Resolved,
    Unresolved,
}

impl PathStore {
    pub fn resolve_mut<'a>(
        &self,
        paths: &'a mut ExploredPaths,
    ) -> &'a mut HashMap<String, HashSet<String>> {
        match self {
            PathStore::Resolved => &mut paths.resolved,
            PathStore::Unresolved => &mut paths.unresolved,
        }
    }

    pub fn resolve<'a>(&self, paths: &'a ExploredPaths) -> &'a HashMap<String, HashSet<String>> {
        match self {
            PathStore::Resolved => &paths.resolved,
            PathStore::Unresolved => &paths.unresolved,
        }
    }
}
