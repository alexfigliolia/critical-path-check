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

#[napi]
pub fn analyzeCriticalPath(html_path: String) -> Result<CriticalPathAnalysis, Error<Status>> {
    with_js_error(|| CriticalPathAnalysis::from(CriticalPathCheck::new(&html_path).run()))
}

#[napi]
pub fn cli(html_path: String) {
    CriticalPathCheck::new(&html_path).run_cli();
}

#[napi]
pub fn measureCriticalPath(html_path: String) -> Result<f64, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).measure() as f64)
}

#[napi]
pub fn assertCriticalCSS(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_css(bytes as usize))
}

#[napi]
pub fn assertCriticalHTML(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_html(bytes as usize))
}

#[napi]
pub fn assertCriticalJavaScript(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert_javascript(bytes as usize))
}

#[napi]
pub fn assertCriticalPath(html_path: String, bytes: f64) -> Result<bool, Error<Status>> {
    with_js_error(|| CriticalPathCheck::new(&html_path).assert(bytes as usize))
}
