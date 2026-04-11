use htmler::{Html, Selector};
use std::{collections::HashSet, path::PathBuf};

use crate::{logger::logger::Logger, parsers::file_paths::FilePaths};

pub struct PathCollector {
    pub file_paths: FilePaths,
    pub css_paths: HashSet<PathBuf>,
    pub javascript_paths: HashSet<PathBuf>,
    pub uncategorized_paths: HashSet<PathBuf>,
}

impl PathCollector {
    pub fn new(build_directory: PathBuf) -> Self {
        PathCollector {
            css_paths: HashSet::new(),
            javascript_paths: HashSet::new(),
            uncategorized_paths: HashSet::new(),
            file_paths: FilePaths::new(build_directory),
        }
    }

    pub fn build(&mut self, html_content: &str) -> &mut Self {
        let document = Html::parse_document(html_content);
        let (script_selector, link_selector) = PathCollector::create_selectors();
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
        if let Some(path_buf) = self.file_paths.to_file_system_path(&path) {
            if path.ends_with(".css") {
                self.css_paths.insert(path_buf);
            } else if path.ends_with(".js") {
                self.javascript_paths.insert(path_buf);
            } else {
                self.uncategorized_paths.insert(path_buf);
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
