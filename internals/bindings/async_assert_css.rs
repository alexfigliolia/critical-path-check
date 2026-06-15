use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
use napi::{Env, Error, Result as NapiResult, Status, Task};
use napi_derive::napi;

use crate::unwind_panic::unwind_panic;

pub struct AsyncAssertCSS {
    pub html_path: String,
    pub threshold: f64,
}

#[napi]
impl Task for AsyncAssertCSS {
    type Output = bool;
    type JsValue = bool;

    fn compute(&mut self) -> Result<Self::Output, Error<Status>> {
        unwind_panic(|| CriticalPathCheck::new(&self.html_path).assert_css(self.threshold as usize))
    }

    fn resolve(&mut self, _: Env, output: bool) -> NapiResult<Self::JsValue> {
        Ok(output)
    }
}
