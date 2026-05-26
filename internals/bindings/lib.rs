use std::panic;

use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
use napi::{Error, Status};
use napi_derive::napi;

use crate::critical_path_analysis::CriticalPathAnalysis;

pub mod critical_path_analysis;

fn with_js_error<F, R>(operation: F) -> Result<R, Error<Status>>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let result = panic::catch_unwind(operation);
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            if let Some(msg) = err.downcast_ref::<&str>() {
                return Err(Error::new(Status::InvalidArg, msg));
            }
            if let Some(msg) = err.downcast_ref::<String>() {
                return Err(Error::new(Status::InvalidArg, msg));
            }
            Err(Error::new(Status::InvalidArg, "Invalid argument"))
        }
    }
}

/// Analyzes the target HTML file's critical render path
///
/// Returns the critical weight (in bytes) required to render your page. It also returns any unresolvable paths that were encountered during the analysis
///
/// #### Arguments
/// * `path` - An absolute path to a root html file
///
/// # Examples
///
/// ```typescript
/// const result = analyzeCriticalPath("/path/to/my/root.html");
/// console.log("Total JS Bytes: {}", result.javascriptWeight);
/// console.log("Total CSS Bytes: {}", result.cssWeight);
/// console.log("Total HTML Bytes: {}", result.htmlWeight);
/// console.log("Unresolved Paths: {}", result.unresolvedPaths);
/// ```
#[napi(js_name = "analyzeCriticalPath")]
pub fn analyze_critical_path(html_path: String) -> Result<CriticalPathAnalysis, Error<Status>> {
    with_js_error(|| CriticalPathAnalysis::from(CriticalPathCheck::new(&html_path).run()))
}

#[napi]
pub fn cli(html_path: String, as_json: Option<bool>) {
    CriticalPathCheck::new(&html_path).run_cli(as_json);
}

/// ## measure
///
/// Returns the combined weight of critical HTML, CSS, and JavaScript
///
/// ```typescript
/// const criticalPathSize = measureCriticalPath("/path/to/index.html");
/// ```
#[napi(js_name = "measureCriticalPath")]
pub fn measure_critical_path(html_path: String) -> Result<f64, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).measure() as f64)
}

/// ## assert_critical_css
///
/// Returns true if the specified number of bytes is less than
/// the byte-weight of critical CSS
///
/// ```typescript
/// const result = assertCriticalCss("/path/to/index.html", 100000);
/// ```
#[napi(js_name = "assertCriticalCss")]
pub fn assert_critical_css(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_css(bytes as usize))
}

/// ## assert_html
///
/// Returns true if the specified number of bytes is less than
/// the byte-weight of critical HTML
///
/// ```typescript
/// const result = assertCriticalHtml("/path/to/index.html", 50000);
/// ```
#[napi(js_name = "assertCriticalHtml")]
pub fn assert_critical_html(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_html(bytes as usize))
}

/// ## assert_critical_javascript
///
/// Returns true if the specified number of bytes is less than
/// the byte-weight of critical javascript
///
/// ```typescript
/// const result = assertCriticalJavaScript("/path/to/index.html", 500000);
/// ```
#[napi(js_name = "assertCriticalJavaScript")]
pub fn assert_critical_javascript(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_javascript(bytes as usize))
}

/// ## assert_critical_path
///
/// Returns true if the specified number of bytes is less than
/// the cummulative critical path
///
/// ```typescript
/// const result = assertCriticalPath("/path/to/index.html", 600000);
/// ```
#[napi(js_name = "assertCriticalPath")]
pub fn assert_critical_path(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert(bytes as usize))
}
