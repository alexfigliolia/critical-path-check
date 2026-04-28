use std::collections::{HashMap, HashSet};

use napi_derive::napi;

use critical_path_check::critical_path_check::critical_path_check::CriticalPathAnalysis as CoreAnalysis;

#[napi(object)]
pub struct CriticalPathAnalysis {
    pub css_weight: f64,
    pub html_weight: f64,
    pub javascript_weight: f64,
    pub unresolved_paths: HashMap<String, HashSet<String>>,
}

impl CriticalPathAnalysis {
    pub fn from(analysis: CoreAnalysis) -> Self {
        CriticalPathAnalysis {
            css_weight: analysis.analysis.css_weight as f64,
            html_weight: analysis.analysis.html_weight as f64,
            javascript_weight: analysis.analysis.javascript_weight as f64,
            unresolved_paths: analysis.unresolved_paths,
        }
    }
}
