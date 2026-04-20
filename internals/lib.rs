use std::path::PathBuf;

use crate::critical_path_check::critical_path_check::{CriticalPathAnalysis, CriticalPathCheck};

pub mod critical_path_check;
mod logger;
pub mod parsers;

/// Analyzes the target HTML file's critical render path
///
/// Returns the critical weight (in bytes) required to render your page. It also returns any unresolvable paths that were encountered during the analysis
///
/// #### Arguments
/// * `path` - An absolute path to a root html file
///
/// # Examples
///
/// ```
/// use critical_path_check::analyze_critical_path;
///
/// let my_html = PathBuf::from("/path/to/my/root.html");
/// let result = analyze_critical_path(&my_html);
/// println!("Total JS Bytes: {}", result.analysis.javascript_weight);
/// println!("Total CSS Bytes: {}", result.analysis.css_weight);
/// println!("Total HTML Bytes: {}", result.analysis.html_weight);
/// ```
pub fn analyze_critical_path(path: &PathBuf) -> CriticalPathAnalysis {
    CriticalPathCheck::from_path_buf(path).run()
}
