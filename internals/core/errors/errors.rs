use std::sync::{LazyLock, Mutex};

pub struct Error {
    reason: String,
}

static HOISTED_ERRORS: LazyLock<Mutex<Vec<Error>>> = LazyLock::new(|| Mutex::new(Vec::new()));

impl Error {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }

    pub fn hoist(error: Error) {
        HOISTED_ERRORS.lock().unwrap().push(error);
    }

    pub fn unload() -> String {
        let mut errors = HOISTED_ERRORS.lock().unwrap();
        let reason_list: Vec<String> = errors.iter().map(|e| e.reason.to_owned()).collect();
        let result = reason_list.join("\n");
        errors.clear();
        errors.shrink_to_fit();
        result
    }
}
