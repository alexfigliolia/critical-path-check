use std::collections::{HashMap, HashSet};

use critical_path_check::critical_path_check::critical_path_check::CriticalPathAnalysis;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CriticalResources {
    pub html_weight: usize,
    pub javascript_weight: usize,
    pub css_weight: usize,
    pub uncategorized_weight: usize,
}

#[wasm_bindgen]
pub struct CriticalPath {
    analysis: CriticalResources,
    unresolved_paths: JsValue,
}

impl CriticalPath {
    pub fn from(analysis: CriticalPathAnalysis) -> Self {
        CriticalPath {
            analysis: CriticalResources {
                javascript_weight: analysis.analysis.javascript_weight,
                css_weight: analysis.analysis.css_weight,
                html_weight: analysis.analysis.html_weight,
                uncategorized_weight: analysis.analysis.uncategorized_weight,
            },
            unresolved_paths: CriticalPath::to_javascript(analysis.unresolved_paths),
        }
    }

    pub fn to_javascript(unresolved_paths: HashMap<String, HashSet<String>>) -> JsValue {
        serde_wasm_bindgen::to_value(&unresolved_paths).unwrap()
    }
}
