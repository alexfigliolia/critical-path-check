use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
use napi::{Env, Error, Result as NapiResult, Status, Task};
use napi_derive::napi;

use crate::unwind_panic::unwind_panic;

pub struct AsyncMeasure {
    pub html_path: String,
}

#[napi]
impl Task for AsyncMeasure {
    type Output = f64;
    type JsValue = f64;

    fn compute(&mut self) -> Result<Self::Output, Error<Status>> {
        unwind_panic(|| CriticalPathCheck::new(&self.html_path).measure() as f64)
    }

    fn resolve(&mut self, _: Env, output: f64) -> NapiResult<Self::JsValue> {
        Ok(output)
    }
}
