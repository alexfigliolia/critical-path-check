use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use fancy_regex::Regex as RegexWithLookAhead;
use normalize_path::NormalizePath;
use regex::Regex;

use crate::visitor::file_resolution_strategy::FileResolutionStrategy;

#[derive(Clone)]
pub struct FilePaths {
    pub root_directory: FileResolutionStrategy,
}

static URL_PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"https?:\/\/.*?\/([^\?]+)"#).unwrap());

static URL_PROTOCOL_AND_PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(.*):\/\/(.*)"#).unwrap());

pub static PRE_QUERY_PARAM_REGEX: LazyLock<RegexWithLookAhead> =
    LazyLock::new(|| RegexWithLookAhead::new(r#"^(?:(.*)(?=\?)|.*)"#).unwrap());

impl FilePaths {
    pub fn new(root_directory: &FileResolutionStrategy) -> Self {
        FilePaths {
            root_directory: root_directory.to_owned(),
        }
    }

    pub fn from_owned(root_directory: FileResolutionStrategy) -> Self {
        FilePaths { root_directory }
    }

    pub fn before_query_params(path: &str) -> &str {
        if let Ok(capture_result) = PRE_QUERY_PARAM_REGEX.captures(path)
            && let Some(first_capture) = capture_result
            && let Some(file_name_without_query_params) = first_capture.get(1)
        {
            return file_name_without_query_params.as_str();
        }
        path
    }

    pub fn to_string(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }

    pub fn resolve_file(
        &self,
        path: &str,
        additional_roots: &Vec<PathBuf>,
    ) -> Option<FileResolutionStrategy> {
        let is_http_path = path.starts_with("http");
        match &self.root_directory {
            FileResolutionStrategy::Http(url) => {
                if is_http_path {
                    return Some(FileResolutionStrategy::Http(url.to_owned()));
                }
                if let Some(captures) = URL_PROTOCOL_AND_PATH_REGEX.captures(url)
                    && let Some(protocol) = captures.get(1)
                    && let Some(origin) = captures.get(2)
                    && let Some(joined) = Path::new(origin.as_str()).join(path).normalize().to_str()
                {
                    return Some(FileResolutionStrategy::Http(format!(
                        "{}://{joined}",
                        protocol.as_str()
                    )));
                }
                None
            }
            FileResolutionStrategy::Local(root_directory) => {
                let mut relative_path = String::from(path);
                if is_http_path
                    && let Some(captures) = URL_PATH_REGEX.captures(&relative_path)
                    && let Some(path) = captures.get(1)
                {
                    relative_path = path.as_str().to_string();
                }
                if relative_path.starts_with("/") {
                    relative_path.remove(0);
                }
                relative_path = FilePaths::before_query_params(&relative_path).to_owned();
                if !relative_path.is_empty() {
                    let mut roots = Vec::from_iter(additional_roots);
                    roots.insert(0, root_directory);
                    for root_dir in roots {
                        let absolute_path = root_dir.join(&relative_path);
                        if absolute_path.exists() {
                            return Some(FileResolutionStrategy::Local(absolute_path.normalize()));
                        }
                    }
                }
                if is_http_path {
                    return Some(FileResolutionStrategy::Http(path.to_owned()));
                }
                None
            }
        }
    }
}
