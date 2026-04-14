use std::path::PathBuf;

use crate::{
    critical_path_check::critical_path_check::{CriticalPathAnalysis, CriticalPathCheck},
    parsers::file_paths::FilePaths,
};

pub mod critical_path_check;
mod logger;
pub mod parsers;

/// Analyzes the target HTML file's critical render path
///
/// Returns the critical weight (in bytes) required to render your page. It also returns any unresolvable paths that were encountered during the analysis
///
/// #### Arguments
/// * `html_path` - An absolute path to an html file
///
/// # Examples
///
/// ```
/// // Code in documentation blocks is compiled and run as a test by rustdoc.
/// use critical_path_check::analyze_critical_path;
///
/// let my_html = PathBuf::from("/path/to/my/root.html");
/// let result = analyze_critical_path(&my_html);
/// ```
pub fn analyze_critical_path(path: &PathBuf) -> CriticalPathAnalysis {
    let analysis = CriticalPathCheck::from(path).run();
    let unresolved_paths = FilePaths::unresolved_paths().clone();
    CriticalPathAnalysis {
        analysis,
        unresolved_paths,
    }
}
