use critical_path_check::critical_path_check::critical_path_check::CriticalPathAnalysis;
use neon::{prelude::*, result::JsResult, types::JsObject};

pub struct CriticalPath {
    analysis: CriticalPathAnalysis,
}

impl CriticalPath {
    pub fn from(analysis: CriticalPathAnalysis) -> Self {
        CriticalPath { analysis }
    }

    pub fn to_javascript<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let html_weight = cx.number(self.analysis.analysis.html_weight as f64);
        obj.set(cx, "htmlWeight", html_weight)?;

        let javascript_weight = cx.number(self.analysis.analysis.javascript_weight as f64);
        obj.set(cx, "javascriptWeight", javascript_weight)?;

        let css_weight = cx.number(self.analysis.analysis.css_weight as f64);
        obj.set(cx, "cssWeight", css_weight)?;

        let unresolved_paths = cx.empty_object();
        for (root, paths) in &self.analysis.unresolved_paths {
            let bucket = cx.empty_array();
            for (index, path) in paths.iter().enumerate() {
                let path_string = cx.string(path);
                bucket.set(cx, index as u32, path_string)?;
            }
            unresolved_paths.set(cx, root.as_str(), bucket)?;
        }
        obj.set(cx, "unresolvedPaths", unresolved_paths)?;

        Ok(obj)
    }
}
