use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

use crate::parsers::file_paths::{FilePaths, FileResolutionStrategy};

static HTTP_FILE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#".*\/([^?]*)"#).unwrap());
static PRE_QUERY_PARAM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(.*)\?"#).unwrap());

pub struct HTMLParser {
    pub file_paths: FilePaths,
    pub html_path: FileResolutionStrategy,
    pub css_paths: HashMap<String, FileResolutionStrategy>,
    pub javascript_paths: HashMap<String, FileResolutionStrategy>,
}

impl HTMLParser {
    pub fn new(
        root_html: &FileResolutionStrategy,
        build_directory: &FileResolutionStrategy,
    ) -> Self {
        HTMLParser {
            css_paths: HashMap::new(),
            javascript_paths: HashMap::new(),
            file_paths: FilePaths::new(build_directory),
            html_path: root_html.to_owned(),
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
            match &resolver {
                FileResolutionStrategy::Http(url) => {
                    if let Some(file_name_match) = HTTP_FILE_REGEX.captures_iter(url).nth(0)
                        && let Some(file_name) = file_name_match.get(1)
                    {
                        let mut file_name_cleaned = file_name.as_str();

                        if let Some(file_without_query_params_match) = PRE_QUERY_PARAM_REGEX
                            .captures_iter(file_name.as_str())
                            .nth(0)
                            && let Some(file_name_without_query_params) =
                                file_without_query_params_match.get(1)
                        {
                            file_name_cleaned = file_name_without_query_params.as_str();
                        }
                        if file_name_cleaned.ends_with(".css")
                            || file_name_cleaned.contains(".css.")
                        {
                            self.css_paths.insert(url.to_owned(), resolver);
                        } else if file_name_cleaned.ends_with(".js")
                            || file_name_cleaned.contains(".js.")
                        {
                            self.javascript_paths.insert(url.to_owned(), resolver);
                        }
                    } else {
                        FilePaths::store_unresolved_path(&self.html_path, path);
                    }
                }
                FileResolutionStrategy::Local(_) => {
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
    }
}
