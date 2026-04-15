use critical_path_check::{
    critical_path_check::critical_path_check::{CriticalPathAnalysis, CriticalPathCheck},
    parsers::file_paths::FilePaths,
};
use napi_derive::napi;

use crate::structs::critical_path::CriticalPath;

pub mod structs;

#[napi]
pub async fn analyzeCriticalPath(path: String) -> CriticalPath {
    let analysis = CriticalPathCheck::new(&path).run();
    let unresolved_paths = FilePaths::unresolved_paths().clone();
    CriticalPath::from(CriticalPathAnalysis {
        analysis,
        unresolved_paths,
    })
}

#[napi]
pub async fn assertCriticalPath(path: String, bytes: i64) -> bool {
    let analysis = CriticalPathCheck::new(&path);
    analysis.assert(bytes as usize)
}

#[napi]
pub async fn assertHTML(path: String, bytes: i64) -> bool {
    let analysis = CriticalPathCheck::new(&path);
    analysis.assert_html(bytes as usize)
}

#[napi]
pub async fn assertCSS(path: String, bytes: i64) -> bool {
    let analysis = CriticalPathCheck::new(&path);
    analysis.assert_css(bytes as usize)
}

#[napi]
pub async fn assertJavaScript(path: String, bytes: i64) -> bool {
    let analysis = CriticalPathCheck::new(&path);
    analysis.assert_javascript(bytes as usize)
}

#[napi]
pub async fn measureCriticalPath(path: String) -> i64 {
    let analysis = CriticalPathCheck::new(&path);
    analysis.measure() as i64
}
