use std::collections::{HashMap, HashSet};

use critical_path_check::critical_path_check::critical_resources::CriticalResources;
use napi_derive::napi;

#[napi(object)]
pub struct CriticalPathAnalysis {
    #[napi(js_name = "cssWeight")]
    pub css_weight: f64,
    #[napi(js_name = "htmlWeight")]
    pub html_weight: f64,
    #[napi(js_name = "javascriptWeight")]
    pub javascript_weight: f64,
    #[napi(js_name = "resolvedPaths")]
    pub resolved_paths: HashMap<String, HashSet<String>>,
    #[napi(js_name = "unresolvedPaths")]
    pub unresolved_paths: HashMap<String, HashSet<String>>,
}

impl CriticalPathAnalysis {
    pub fn from(analysis: CriticalResources) -> Self {
        CriticalPathAnalysis {
            css_weight: analysis.css_weight as f64,
            html_weight: analysis.html_weight as f64,
            javascript_weight: analysis.javascript_weight as f64,
            resolved_paths: analysis.paths.resolved,
            unresolved_paths: analysis.paths.unresolved,
        }
    }
}
