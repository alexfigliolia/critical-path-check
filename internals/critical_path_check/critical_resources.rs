use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    path::PathBuf,
    process::exit,
};


use crate::{
    logger::logger::Logger,
    parsers::{
        css_parser::CSSParser, file_paths::FileResolutionStrategy, html_parser::HTMLParser,
        javascript_parser::JavaScriptParser, traverser::Traverser,
    },
};

pub struct CriticalResources {
    pub root_html: PathBuf,
    pub html_weight: usize,
    pub javascript_weight: usize,
    pub css_weight: usize,
}

impl CriticalResources {
    pub fn new(root_html: &PathBuf) -> Self {
        CriticalResources {
            html_weight: 0,
            javascript_weight: 0,
            css_weight: 0,
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
        let build_directory = self.html_directory();
        let mut html_parser = HTMLParser::new(&self.root_html, &build_directory);
        html_parser.build(&html_content);
        self.javascript_weight = JavaScriptParser::new(
            &build_directory,
            self.to_stack(&html_parser.javascript_paths),
        )
        .traverse();
        self.css_weight =
            CSSParser::new(&build_directory, self.to_stack(&html_parser.css_paths)).traverse();
    }

    pub fn total_weight(&self) -> usize {
        self.html_weight + self.javascript_weight + self.css_weight
    }

    pub fn log_stats(&self) {
        Logger::info("Collecting totals");
        println!();
        Logger::green("JavaScript Weight");
        Logger::log_stat(self.javascript_weight);
        Logger::green("CSS Weight");
        Logger::log_stat(self.css_weight);
        Logger::green("HTML Weight");
        Logger::log_stat(self.html_weight);
        let total = self.total_weight();
        Logger::green("Combined");
        Logger::log_stat(total);
    }

    fn html_directory(&self) -> PathBuf {
        if let Some(parent_dir) = self.root_html.parent() {
            return parent_dir.to_path_buf();
        }
        Logger::panic_with_error("I was unable to determine the HTML's directory");
        exit(1);
    }

    fn to_stack(
        &self,
        paths: &HashMap<String, FileResolutionStrategy>,
    ) -> VecDeque<(FileResolutionStrategy, FileResolutionStrategy)> {
        VecDeque::from_iter(paths.values().map(|entry| {
            (
                entry.to_owned(),
                FileResolutionStrategy::Local(self.root_html.clone()),
            )
        }))
    }
}
