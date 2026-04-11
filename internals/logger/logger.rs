use colored::{ColoredString, Colorize};

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!("{}{}", Logger::info_prefix(), message);
    }

    pub fn error(message: &str) {
        println!("{}{}", Logger::error_prefix(), message);
    }

    pub fn panic_with_error(message: &str) {
        Logger::error(message);
        panic!();
    }

    fn info_prefix() -> ColoredString {
        "CP Check: ".bright_blue().bold()
    }

    fn error_prefix() -> ColoredString {
        "CP Check: ".red().bold()
    }
}
