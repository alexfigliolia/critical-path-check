use crate::{
    logger::logger::Logger,
    parsers::{
        file_paths::{FilePaths, FileResolutionStrategy},
        html_parser::HTMLParser,
    },
};

pub struct CriticalResources {
    pub css_weight: usize,
    pub html_weight: usize,
    pub javascript_weight: usize,
}

impl CriticalResources {
    pub fn new(root_html: &FileResolutionStrategy) -> Self {
        HTMLParser::new(root_html).traverse_linked_resources()
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
}
