use std::collections::{HashMap, HashSet};

use critical_path_check::critical_path_check::critical_path_check::CriticalPathAnalysis;
use napi_derive::napi;

#[napi(object)]
pub struct CriticalResources {
    pub html_weight: i64,
    pub javascript_weight: i64,
    pub css_weight: i64,
    pub uncategorized_weight: i64,
}

#[napi(object)]
pub struct CriticalPath {
    pub analysis: CriticalResources,
    pub unresolved_paths: HashMap<String, HashSet<String>>,
}

impl CriticalPath {
    pub fn from(analysis: CriticalPathAnalysis) -> Self {
        CriticalPath {
            analysis: CriticalResources {
                javascript_weight: analysis.analysis.javascript_weight as i64,
                css_weight: analysis.analysis.css_weight as i64,
                html_weight: analysis.analysis.html_weight as i64,
                uncategorized_weight: analysis.analysis.uncategorized_weight as i64,
            },
            unresolved_paths: analysis.unresolved_paths,
        }
    }
}
