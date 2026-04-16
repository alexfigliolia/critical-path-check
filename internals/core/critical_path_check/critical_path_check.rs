use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::PathBuf,
};

use colored::Colorize;

use crate::{
    critical_path_check::critical_resources::CriticalResources, logger::logger::Logger,
    parsers::file_paths::FilePaths,
};

pub struct CriticalPathCheck {
    root_html: PathBuf,
}

/// Utilities for analyzing the critical render path of a web application
///
/// # Examples
///
/// ```
/// use critical_path_check::critical_path_check::CriticalPathCheck;
///
/// let cp_check = CriticalPathCheck::new("<absolute-path-to-html-str>");
/// # or
/// let cp_check = CriticalPathCheck::from(&PathBuf::from("/path/to/my/root.html"));
/// let result = cp_check.run();
/// println!("Total JS Bytes: {}", result.analysis.javascript_weight);
/// println!("Total CSS Bytes: {}", result.analysis.css_weight);
/// println!("Total HTML Bytes: {}", result.analysis.html_weight);
/// println!("Total Bytes: {}", cp_check.measure());
/// ```
pub struct CriticalPathAnalysis {
    pub analysis: CriticalResources,
    pub unresolved_paths: HashMap<String, HashSet<String>>,
}

impl CriticalPathCheck {
    pub fn new(root_html: &str) -> Self {
        CriticalPathCheck {
            root_html: CriticalPathCheck::validate_path_string(root_html),
        }
    }

    pub fn from(root_html: &PathBuf) -> Self {
        CriticalPathCheck {
            root_html: CriticalPathCheck::validate_path(root_html),
        }
    }

    pub fn assert(&self, bytes: usize) -> bool {
        self.measure() < bytes
    }

    pub fn assert_javascript(&self, bytes: usize) -> bool {
        self.run().analysis.javascript_weight < bytes
    }

    pub fn assert_css(&self, bytes: usize) -> bool {
        self.run().analysis.css_weight < bytes
    }

    pub fn assert_html(&self, bytes: usize) -> bool {
        self.run().analysis.html_weight < bytes
    }

    pub fn measure(&self) -> usize {
        let graph = self.run();
        graph.analysis.total_weight()
    }

    pub fn run(&self) -> CriticalPathAnalysis {
        let mut analysis = CriticalResources::new(&self.root_html);
        analysis.build();
        let unresolved_paths = FilePaths::unresolved_paths().clone();
        FilePaths::clear_unresolved_paths();
        CriticalPathAnalysis {
            analysis,
            unresolved_paths,
        }
    }

    pub fn run_cli(&self) {
        let mut analysis = CriticalResources::new(&self.root_html);
        analysis.build();
        FilePaths::log_unresolved();
        analysis.log_stats();
        FilePaths::clear_unresolved_paths();
    }

    fn validate_path_string(root_html: &str) -> PathBuf {
        let path = PathBuf::from(root_html);
        CriticalPathCheck::validate_path(&path)
    }

    fn validate_path(path: &PathBuf) -> PathBuf {
        if !path.is_absolute() {
            Logger::panic_with_error(
                "Your input path must be an absolute path to your root HTML file",
            );
        }
        if !path.exists() {
            Logger::panic_with_error("The specified input path does not exist");
        }
        if path.is_dir() || path.extension().unwrap_or(OsStr::new("")) != "html" {
            Logger::panic_with_error("The specified input does not point to an html file");
        }
        Logger::info(format!("Analyzing {}", FilePaths::to_string(path).bright_blue()).as_str());
        path.to_path_buf()
    }
}
