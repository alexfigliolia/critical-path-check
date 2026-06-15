use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
use napi::{Env, Error, Result as NapiResult, Status, Task};
use napi_derive::napi;

use crate::unwind_panic::unwind_panic;

pub struct AsyncCLI {
    pub html_path: String,
    pub as_json: Option<bool>,
}

#[napi]
impl Task for AsyncCLI {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> Result<Self::Output, Error<Status>> {
        unwind_panic(|| CriticalPathCheck::new(&self.html_path).run_cli(self.as_json))
    }

    fn resolve(&mut self, _: Env, output: ()) -> NapiResult<Self::JsValue> {
        Ok(output)
    }
}
