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
