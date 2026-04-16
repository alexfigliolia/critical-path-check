use critical_path_check::{
    critical_path_check::critical_path_check::{CriticalPathAnalysis, CriticalPathCheck},
    parsers::file_paths::FilePaths,
};
use neon::{
    prelude::{Context, FunctionContext},
    result::JsResult,
    types::{JsBoolean, JsObject},
};

use crate::structs::critical_path::CriticalPath;

pub mod structs;

#[neon::export]
pub fn analyzeCriticalPath<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
) -> JsResult<'a, JsObject> {
    let analysis = CriticalPathCheck::new(&path).run();
    let unresolved_paths = FilePaths::unresolved_paths().clone();
    FilePaths::clear_unresolved_paths();
    CriticalPath::from(CriticalPathAnalysis {
        analysis,
        unresolved_paths,
    })
    .to_javascript(cx)
}

#[neon::export]
pub fn assertCriticalPath<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert(bytes as usize)))
}

#[neon::export]
pub fn assertHTML<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert_html(bytes as usize)))
}

#[neon::export]
pub fn assertCSS<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert_css(bytes as usize)))
}

#[neon::export]
pub fn assertJavaScript<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert_javascript(bytes as usize)))
}

#[neon::export]
pub fn measureCriticalPath<'a>(path: String) -> f64 {
    let analysis = CriticalPathCheck::new(&path);
    analysis.measure() as f64
}
