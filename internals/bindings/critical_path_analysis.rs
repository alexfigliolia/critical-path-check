use std::collections::{HashMap, HashSet};

use napi_derive::napi;

use critical_path_check::critical_path_check::critical_path_check::CriticalPathAnalysis as CoreAnalysis;

#[napi(object)]
pub struct CriticalPathAnalysis {
    #[napi(js_name = "cssWeight")]
    pub css_weight: f64,
    #[napi(js_name = "htmlWeight")]
    pub html_weight: f64,
    #[napi(js_name = "javascriptWeight")]
    pub javascript_weight: f64,
    #[napi(js_name = "unresolvedPaths")]
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
