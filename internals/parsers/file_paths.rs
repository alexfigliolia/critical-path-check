use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{LazyLock, Mutex, MutexGuard},
};

use normalize_path::NormalizePath;
use url::Url;

#[derive(Clone)]
pub struct FilePaths {
    root_directory: PathBuf,
}

static UNRESOLVED_PATHS: LazyLock<Mutex<HashMap<PathBuf, HashSet<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl FilePaths {
    pub fn new(root_directory: PathBuf) -> Self {
        FilePaths { root_directory }
    }

    pub fn unresolved_paths() -> MutexGuard<'static, HashMap<PathBuf, HashSet<String>>> {
        UNRESOLVED_PATHS.lock().unwrap()
    }

    pub fn store_unresolved_path(root: &PathBuf, path: &str) {
        let mut unresolved = FilePaths::unresolved_paths();
        if let Some(bucket) = unresolved.get_mut(root) {
            bucket.insert(path.to_owned());
            let thing: HashSet<String> = bucket.clone();
            unresolved.insert(root.to_owned(), thing);
        }
    }

    pub fn to_file_system_path(&self, path: &str) -> Option<PathBuf> {
        if path.starts_with("http")
            && let Ok(url) = Url::parse(path)
        {
            return self.to_existent_path_buf(url.path(), true);
        }
        self.to_existent_path_buf(path, false)
    }

    pub fn to_existent_path_buf(&self, path: &str, find: bool) -> Option<PathBuf> {
        if path.is_empty() {
            return None;
        }
        let mut relative_path = String::from(path);
        if relative_path.starts_with("/") {
            relative_path.remove(0);
        }
        let fs_path = self.root_directory.join(relative_path).normalize();
        if !fs_path.exists() {
            FilePaths::store_unresolved_path(&self.root_directory, path);
            if find {
                // TODO attempt to find a matching path
                return None;
            }
            return None;
        }
        Some(fs_path)
    }

    pub fn to_string(path: &PathBuf) -> String {
        path.to_string_lossy().to_string()
    }
}
