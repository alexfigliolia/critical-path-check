use std::panic;

use napi::{Error, Status};

pub fn unwind_panic<F, R>(operation: F) -> Result<R, Error<Status>>
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
