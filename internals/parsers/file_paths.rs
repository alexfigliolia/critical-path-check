use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex, MutexGuard},
};

use colored::Colorize;
use normalize_path::NormalizePath;
use url::Url;

use crate::logger::logger::Logger;

#[derive(Clone)]
pub enum FileResolutionStrategy {
    Http(String),
    Local(PathBuf),
}

#[derive(Clone)]
pub struct FilePaths {
    pub root_directory: PathBuf,
}

static UNRESOLVED_PATHS: LazyLock<Mutex<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl FilePaths {
    pub fn new(root_directory: &PathBuf) -> Self {
        FilePaths {
            root_directory: root_directory.to_owned(),
        }
    }

    pub fn unresolved_paths() -> MutexGuard<'static, HashMap<String, HashSet<String>>> {
        UNRESOLVED_PATHS.lock().unwrap()
    }

    pub fn store_unresolved_path(root: &FileResolutionStrategy, path: &str) {
        let mut unresolved = FilePaths::unresolved_paths();
        let root_hash = FilePaths::hash(root);
        let mut default_bucket = HashSet::new();
        let bucket = unresolved
            .get_mut(&root_hash)
            .unwrap_or(&mut default_bucket);
        bucket.insert(path.to_owned());
        let thing: HashSet<String> = bucket.clone();
        unresolved.insert(root_hash, thing);
    }

    pub fn to_string(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }

    pub fn resolve_file(
        &self,
        path: &str,
        additional_roots: &Vec<PathBuf>,
    ) -> Option<FileResolutionStrategy> {
        let mut relative_path = String::from(path);
        if path.starts_with("http")
            && let Ok(url) = Url::parse(path)
        {
            relative_path = url.path().to_string();
        }
        if relative_path.starts_with("/") {
            relative_path.remove(0);
        }
        if !relative_path.is_empty() {
            let mut roots = Vec::from_iter(additional_roots);
            roots.insert(0, &self.root_directory);
            for root_directory in roots {
                let absolute_path = root_directory.join(&relative_path).normalize();
                if absolute_path.exists() {
                    return Some(FileResolutionStrategy::Local(absolute_path));
                }
            }
        }
        if path.starts_with("http") {
            return Some(FileResolutionStrategy::Http(path.to_owned()));
        }
        None
    }

    pub async fn read_resource(path: &PathBuf) -> Option<String> {
        if let Ok(content) = read_to_string(path) {
            return Some(content);
        }
        None
    }

    pub async fn fetch_resource(url: &str) -> Option<String> {
        if let Ok(response) = reqwest::get(url).await
            && let Ok(text) = response.text().await
        {
            return Some(text);
        }
        None
    }

    pub fn hash(strategy: &FileResolutionStrategy) -> String {
        match strategy {
            FileResolutionStrategy::Http(url) => url.to_owned(),
            FileResolutionStrategy::Local(path) => FilePaths::to_string(path),
        }
    }

    pub fn log_unresolved() {
        let unresolved = FilePaths::unresolved_paths();
        if !unresolved.is_empty() {
            println!();
            Logger::info(
                "The following file references were not resolved and will be omitted from analysis",
            );
            for (root, bucket) in unresolved.iter() {
                println!(
                    "\n{}{}{}",
                    Logger::indent(None),
                    "Origin: ".cyan(),
                    root.bright_blue()
                );
                for path in bucket {
                    println!(
                        "{}{}{}",
                        Logger::indent(Some(6)),
                        "Referenced: ".cyan(),
                        path.bright_blue()
                    )
                }
            }
            println!();
        }
    }
}
