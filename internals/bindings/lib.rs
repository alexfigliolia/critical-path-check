use napi::bindgen_prelude::AsyncTask;
use napi_derive::napi;

use crate::{
    async_analyze::AsyncAnalyze, async_assert::AsyncAssert, async_assert_css::AsyncAssertCSS,
    async_assert_html::AsyncAssertHTML, async_assert_javascript::AsyncAssertJavaScript,
    async_cli::AsyncCLI, async_measure::AsyncMeasure,
};

mod async_analyze;
mod async_assert;
mod async_assert_css;
mod async_assert_html;
mod async_assert_javascript;
mod async_cli;
mod async_measure;
pub mod critical_path_analysis;
mod unwind_panic;

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
/// import { analyzeCriticalPath } from "@ui-perf/critical-path";
///
/// const result = await analyzeCriticalPath("/path/to/my/root.html");
///
/// console.log("Total JS Bytes: {}", result.javascriptWeight);
/// console.log("Total CSS Bytes: {}", result.cssWeight);
/// console.log("Total HTML Bytes: {}", result.htmlWeight);
/// console.log("Resolved Paths: {}", result.resolvedPaths);
/// console.log("Unresolved Paths: {}", result.unresolvedPaths);
/// ```
#[napi(js_name = "analyzeCriticalPath")]
pub fn analyze_critical_path(html_path: String) -> AsyncTask<AsyncAnalyze> {
    AsyncTask::new(AsyncAnalyze { html_path })
}

#[napi]
pub fn cli(html_path: String, as_json: Option<bool>) -> AsyncTask<AsyncCLI> {
    AsyncTask::new(AsyncCLI { html_path, as_json })
}

/// ## measure
///
/// Returns the combined weight of critical HTML, CSS, and JavaScript
///
/// ```typescript
/// import { measureCriticalPath } from "@ui-perf/critical-path";
///
/// const criticalPathSize = await measureCriticalPath("/path/to/index.html");
/// ```
#[napi(js_name = "measureCriticalPath")]
pub fn measure_critical_path(html_path: String) -> AsyncTask<AsyncMeasure> {
    AsyncTask::new(AsyncMeasure { html_path })
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
pub fn assert_critical_css(html_path: String, threshold: f64) -> AsyncTask<AsyncAssertCSS> {
    AsyncTask::new(AsyncAssertCSS {
        html_path,
        threshold,
    })
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
pub fn assert_critical_html(html_path: String, threshold: f64) -> AsyncTask<AsyncAssertHTML> {
    AsyncTask::new(AsyncAssertHTML {
        html_path,
        threshold,
    })
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
pub fn assert_critical_javascript(
    html_path: String,
    threshold: f64,
) -> AsyncTask<AsyncAssertJavaScript> {
    AsyncTask::new(AsyncAssertJavaScript {
        html_path,
        threshold,
    })
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
pub fn assert_critical_path(html_path: String, threshold: f64) -> AsyncTask<AsyncAssert> {
    AsyncTask::new(AsyncAssert {
        html_path,
        threshold,
    })
}
