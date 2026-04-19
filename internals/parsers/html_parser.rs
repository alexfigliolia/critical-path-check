use regex::Regex;
use std::{collections::HashMap, path::PathBuf};

use crate::parsers::file_paths::{FilePaths, FileResolutionStrategy};

pub struct HTMLParser {
    pub file_paths: FilePaths,
    pub html_path: FileResolutionStrategy,
    pub css_paths: HashMap<String, FileResolutionStrategy>,
    pub javascript_paths: HashMap<String, FileResolutionStrategy>,
}

impl HTMLParser {
    pub fn new(root_html: &PathBuf, build_directory: &PathBuf) -> Self {
        HTMLParser {
            css_paths: HashMap::new(),
            javascript_paths: HashMap::new(),
            file_paths: FilePaths::new(build_directory),
            html_path: FileResolutionStrategy::Local(root_html.to_owned()),
        }
    }

    pub fn build(&mut self, content: &str) {
        self.iterate_matches(
            Regex::new(r#"<script.*?src=['"]([^'"]+)['"].*?>"#).unwrap(),
            content,
        );
        self.iterate_matches(
            Regex::new(r#"<link.*?rel=['"]stylesheet["'].*?href=['"]([^'"]+)['"].*?>"#).unwrap(),
            content,
        );
    }

    fn iterate_matches(&mut self, regex: Regex, content: &str) {
        let captures = regex.captures_iter(content);
        for capture in captures {
            if let Some(group) = capture.get(1) {
                self.index_path(group.as_str());
            }
        }
    }

    fn index_path(&mut self, path: &str) {
        if let Some(resolver) = self.file_paths.resolve_file(path, &Vec::new()) {
            let hash = FilePaths::hash(&resolver);
            if path.ends_with(".css") {
                self.css_paths.insert(hash, resolver);
            } else if path.ends_with(".js") {
                self.javascript_paths.insert(hash, resolver);
            } else {
                FilePaths::store_unresolved_path(&self.html_path, path);
            }
        }
    }
}
