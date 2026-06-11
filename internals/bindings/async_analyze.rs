use critical_path_check::critical_path_check::critical_path_check::CriticalPathCheck;
use napi::{Env, Error, Result as NapiResult, Status, Task};
use napi_derive::napi;

use crate::{critical_path_analysis::CriticalPathAnalysis, unwind_panic::unwind_panic};

pub struct AsyncAnalyze {
    pub html_path: String,
}

#[napi]
impl Task for AsyncAnalyze {
    type Output = CriticalPathAnalysis;
    type JsValue = CriticalPathAnalysis;

    fn compute(&mut self) -> Result<Self::Output, Error<Status>> {
        unwind_panic(|| CriticalPathAnalysis::from(CriticalPathCheck::new(&self.html_path).run()))
    }

    fn resolve(&mut self, _: Env, output: CriticalPathAnalysis) -> NapiResult<Self::JsValue> {
        Ok(output)
    }
}
