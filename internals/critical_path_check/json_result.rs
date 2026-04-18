use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use serde::Serialize;
use serde_json::to_string;

use crate::{
    critical_path_check::critical_resources::CriticalResources, logger::logger::Logger,
    parsers::file_paths::FilePaths,
};

#[derive(Serialize)]
pub struct JSONResult {
    pub htmlWeight: usize,
    pub javascriptWeight: usize,
    pub cssWeight: usize,
    pub unresolvedPaths: HashMap<String, HashSet<String>>,
}

impl JSONResult {
    pub fn from(resources: CriticalResources) -> Self {
        let unresolved_paths = FilePaths::unresolved_paths().clone();
        FilePaths::clear_unresolved_paths();
        JSONResult {
            htmlWeight: resources.html_weight,
            javascriptWeight: resources.javascript_weight,
            cssWeight: resources.css_weight,
            unresolvedPaths: unresolved_paths,
        }
    }

    pub fn to_string(&self) -> String {
        if let Ok(json_string) = to_string(self) {
            return json_string;
        }
        Logger::error("Failed to serialize results");
        Logger::error("Please file a bug to");
        Logger::file_issue();
        exit(1);
    }
}
