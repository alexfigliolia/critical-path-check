use std::{fs::read_to_string, path::PathBuf};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum FileResolutionStrategy {
    Http(String),
    Local(PathBuf),
}

impl FileResolutionStrategy {
    pub fn to_string(&self) -> String {
        match self {
            FileResolutionStrategy::Http(url) => url.to_owned(),
            FileResolutionStrategy::Local(path) => path.to_string_lossy().into(),
        }
    }

    pub fn resolve_file(&self) -> Option<String> {
        if let Some(content) = match self {
            FileResolutionStrategy::Http(url) => FileResolutionStrategy::fetch_resource(url),
            FileResolutionStrategy::Local(path) => FileResolutionStrategy::read_resource(path),
        } {
            return Some(content);
        }
        None
    }

    pub fn read_resource(path: &PathBuf) -> Option<String> {
        if let Ok(content) = read_to_string(path) {
            return Some(content);
        }
        None
    }

    pub fn fetch_resource(url: &str) -> Option<String> {
        if let Ok(response) = minreq::get(url).send()
            && let Ok(body) = response.as_str()
        {
            return Some(body.to_owned());
        }
        None
    }
}
