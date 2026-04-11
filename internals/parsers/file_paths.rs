use std::path::PathBuf;

use normalize_path::NormalizePath;
use url::Url;

#[derive(Clone)]
pub struct FilePaths {
    root_directory: PathBuf,
}

impl FilePaths {
    pub fn new(root_directory: PathBuf) -> Self {
        FilePaths { root_directory }
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
            if find {
                // TODO attempt to find a matching path
                return None;
            }
            return None;
        }
        Some(fs_path)
    }
}
