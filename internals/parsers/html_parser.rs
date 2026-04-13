use htmler::{Html, Selector};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    logger::logger::Logger,
    parsers::file_paths::{FilePaths, FileResolutionStrategy},
};

pub struct HTMLParser {
    pub file_paths: FilePaths,
    pub css_paths: HashMap<String, FileResolutionStrategy>,
    pub javascript_paths: HashMap<String, FileResolutionStrategy>,
    pub uncategorized_paths: HashMap<String, FileResolutionStrategy>,
}

impl HTMLParser {
    pub fn new(build_directory: &PathBuf) -> Self {
        HTMLParser {
            css_paths: HashMap::new(),
            javascript_paths: HashMap::new(),
            uncategorized_paths: HashMap::new(),
            file_paths: FilePaths::new(build_directory),
        }
    }

    pub fn build(&mut self, html_content: &str) -> &mut Self {
        let document = Html::parse_document(html_content);
        let (script_selector, link_selector) = HTMLParser::create_selectors();
        let scripts = document.select(&script_selector);
        let links = document.select(&link_selector);
        for element in scripts {
            if element.has_attribute("src") {
                self.index_path(element.get_attribute("src").to_owned());
            }
        }
        for element in links {
            if element.has_attribute("href") {
                self.index_path(element.get_attribute("href").to_owned());
            }
        }
        self
    }

    fn index_path(&mut self, path: String) {
        if let Some(resolver) = self.file_paths.resolve_file(&path, &Vec::new()) {
            let hash = FilePaths::hash(&resolver);
            if path.ends_with(".css") {
                self.css_paths.insert(hash, resolver);
            } else if path.ends_with(".js") {
                self.javascript_paths.insert(hash, resolver);
            } else {
                self.uncategorized_paths.insert(hash, resolver);
            }
        }
    }

    fn create_selectors() -> (Selector, Selector) {
        if let Ok(scripts) = Selector::parse("script")
            && let Ok(links) = Selector::parse("link")
        {
            return (scripts, links);
        }
        Logger::panic_with_error("Internal error - file bug");
        panic!()
    }
}
