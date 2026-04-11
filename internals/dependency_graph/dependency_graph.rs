use std::{fs::read_to_string, path::PathBuf};

use crate::{
    dependency_graph::path_collector::PathCollector,
    logger::logger::Logger,
    parsers::{javascript_parser::JavaScriptParser, traverser::Traverser},
};

pub struct DependencyGraph {
    pub root_html: PathBuf,
    pub html_weight: usize,
    pub javascript_weight: usize,
    pub css_weight: usize,
    pub uncategorized_weight: usize,
}

impl DependencyGraph {
    pub fn new(root_html: &PathBuf) -> Self {
        DependencyGraph {
            html_weight: 0,
            javascript_weight: 0,
            css_weight: 0,
            uncategorized_weight: 0,
            root_html: root_html.to_owned(),
        }
    }

    pub fn build(&mut self) {
        let result = read_to_string(&self.root_html);
        if let Err(error) = &result {
            Logger::panic_with_error(
                format!("Failed to parse the root HTML file {}", error).as_str(),
            );
        }
        let html_content = result.unwrap();
        self.html_weight += html_content.len();
        let mut collector = PathCollector::new(self.html_directory());
        collector.build(&html_content);
        self.javascript_weight =
            JavaScriptParser::new(self.html_directory(), &collector.javascript_paths).traverse();
    }

    pub fn total_weight(&self) -> usize {
        self.html_weight + self.javascript_weight + self.css_weight + self.uncategorized_weight
    }

    fn html_directory(&self) -> PathBuf {
        if let Some(parent_dir) = self.root_html.parent() {
            return parent_dir.to_path_buf();
        }
        Logger::panic_with_error("I was unable to determine the HTML's directory");
        panic!();
    }
}
