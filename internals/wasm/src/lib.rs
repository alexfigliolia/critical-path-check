use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
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
    CriticalPath::from(analysis).to_javascript(cx)
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
pub fn assertCriticalHTML<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert_html(bytes as usize)))
}

#[neon::export]
pub fn assertCriticalCSS<'a>(
    cx: &mut FunctionContext<'a>,
    path: String,
    bytes: f64,
) -> JsResult<'a, JsBoolean> {
    let analysis = CriticalPathCheck::new(&path);
    Ok(cx.boolean(analysis.assert_css(bytes as usize)))
}

#[neon::export]
pub fn assertCriticalJavaScript<'a>(
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

#[neon::export]
pub fn cli<'a>(path: String) {
    let analysis = CriticalPathCheck::new(&path);
    analysis.run_cli();
}
