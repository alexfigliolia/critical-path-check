use critical_path_check::{
    critical_path_check::critical_path_check::{CriticalPathAnalysis, CriticalPathCheck},
    parsers::file_paths::FilePaths,
};
use wasm_bindgen::prelude::*;

use crate::structs::critical_path::CriticalPath;

pub mod structs;

#[wasm_bindgen]
pub fn analyzeCriticalPath(path: &str) -> CriticalPath {
    let analysis = CriticalPathCheck::new(path).run();
    let unresolved_paths = FilePaths::unresolved_paths().clone();
    CriticalPath::from(CriticalPathAnalysis {
        analysis,
        unresolved_paths,
    })
}
