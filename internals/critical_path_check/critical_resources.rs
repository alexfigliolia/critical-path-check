use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    path::PathBuf,
    process::exit,
};

use regex::Regex;

use crate::{
    logger::logger::Logger,
    parsers::{
        critical_path_parser::CriticalPathParser,
        file_paths::{FilePaths, FileResolutionStrategy},
        html_parser::HTMLParser,
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
        self.javascript_weight = CriticalPathParser::new(
            &build_directory,
            self.to_stack(&html_parser.javascript_paths),
            Regex::new(r#"(import\s*?|from\s*?)['"]([^'"]+)['"]"#).unwrap(),
            2,
        )
        .analyze();
        self.css_weight = CriticalPathParser::new(
            &build_directory,
            self.to_stack(&html_parser.css_paths),
            Regex::new(r#"@(import\s*?(url\()?)['"]([^'"]+)['"]"#).unwrap(),
            3,
        )
        .analyze();
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

    pub fn to_json_str(&self) -> String {
        let mut json_tokens: Vec<String> = Vec::new();
        json_tokens.push("{".to_string());
        json_tokens.push(format!("\"htmlWeight\":{},", self.html_weight));
        json_tokens.push(format!("\"cssWeight\":{},", self.css_weight));
        json_tokens.push(format!("\"javascriptWeight\":{},", self.javascript_weight));
        json_tokens.push("\"unresolvedPaths\": {".to_string());
        let unresolved_paths = FilePaths::unresolved_paths();
        if !unresolved_paths.is_empty() {
            let max_root_index = unresolved_paths.len() - 1;
            for (root_idx, (root, paths)) in unresolved_paths.iter().enumerate() {
                json_tokens.push(format!("\"{}\":[", root));
                let max_paths_index = paths.len() - 1;
                for (path_idx, path) in paths.iter().enumerate() {
                    if path_idx == max_paths_index {
                        json_tokens.push(format!("\"{path}\""));
                    } else {
                        json_tokens.push(format!("\"{path}\","));
                    }
                }
                if root_idx == max_root_index {
                    json_tokens.push("]".to_string());
                } else {
                    json_tokens.push("],".to_string());
                }
            }
        }
        json_tokens.push("}}".to_string());
        json_tokens.join("")
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
