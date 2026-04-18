use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::{
    critical_path_check::{critical_resources::CriticalResources, json_result::JSONResult},
    logger::logger::Logger,
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
    /// ## new
    ///
    /// Given an `&str` representing an absolute path to an HTML file,
    /// spawns an instance of the `CriticalPathCheck` scoped to that
    /// file
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// ```
    pub fn new(root_html: &str) -> Self {
        CriticalPathCheck {
            root_html: CriticalPathCheck::validate_path_string(root_html),
        }
    }

    /// ## from
    ///
    /// Given an `&PathBuf` with an absolute path to an HTML file,
    /// spawns an instance of the `CriticalPathCheck` scoped to that
    /// file
    ///
    /// ```rust
    /// let my_path = PathBuf::from("/path/to/index.html");
    /// let cp_check = CriticalPathCheck::from(&my_path);
    /// ```
    pub fn from(root_html: &PathBuf) -> Self {
        CriticalPathCheck {
            root_html: CriticalPathCheck::validate_path(root_html),
        }
    }

    /// ## assert
    ///
    /// Returns true if the specified number of bytes is greater than
    /// the cummulative critical path
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let check_passed = cp_check.assert(600000);
    /// ```
    pub fn assert(&self, bytes: usize) -> bool {
        self.measure() < bytes
    }

    /// ## assert_javascript
    ///
    /// Returns true if the specified number of bytes is greater than
    /// the byte-weight of critical javascript
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let check_passed = cp_check.assert_javascript(500000);
    /// ```
    pub fn assert_javascript(&self, bytes: usize) -> bool {
        self.run().analysis.javascript_weight < bytes
    }

    /// ## assert_css
    ///
    /// Returns true if the specified number of bytes is greater than
    /// the byte-weight of critical CSS
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let check_passed = cp_check.assert_css(100000);
    /// ```
    pub fn assert_css(&self, bytes: usize) -> bool {
        self.run().analysis.css_weight < bytes
    }

    /// ## assert_html
    ///
    /// Returns true if the specified number of bytes is greater than
    /// the byte-weight of critical HTML
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let check_passed = cp_check.assert_html(50000);
    /// ```
    pub fn assert_html(&self, bytes: usize) -> bool {
        self.run().analysis.html_weight < bytes
    }

    /// ## measure
    ///
    /// Returns the combined weight of critical HTML, CSS, and JavaScript
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let critical_path_size = cp_check.measure();
    /// ```
    pub fn measure(&self) -> usize {
        let graph = self.run();
        graph.analysis.total_weight()
    }

    /// ## run
    ///
    /// Returns a critical path analysis containing the byte-weights of
    /// critical HTML, CSS, and JavaScript as well as any unresolvable
    /// imports/references encountered
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// let result = cp_check.run();
    /// ```
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

    /// ## run_cli
    ///
    /// Executes the critical path analysis as a CLI command logging all
    /// results to `stdout`
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// cp_check.run_cli();
    /// ```
    pub fn run_cli(&self) {
        let mut analysis = CriticalResources::new(&self.root_html);
        analysis.build();
        FilePaths::log_unresolved();
        analysis.log_stats();
        FilePaths::clear_unresolved_paths();
    }

    /// ## as_json
    ///
    /// Executes the critical path analysis as a CLI command logging the
    /// analysis results as a JSON object to stdout
    ///
    /// ```rust
    /// let cp_check = CriticalPathCheck::new("/path/to/index.html");
    /// cp_check.as_json();
    /// ```
    pub fn as_json(&self) {
        let mut analysis = CriticalResources::new(&self.root_html);
        analysis.build();
        let json_result = JSONResult::from(analysis);
        println!("{}", json_result.to_string());
    }

    fn validate_path_string(root_html: &str) -> PathBuf {
        let path = PathBuf::from(root_html);
        CriticalPathCheck::validate_path(&path)
    }

    fn validate_path(path: &Path) -> PathBuf {
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
