use colored::{ColoredString, Colorize};

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!("{}{}", Logger::info_prefix(), message);
    }

    pub fn error(message: &str) {
        println!("{}{}", Logger::error_prefix(), message);
    }

    pub fn green(message: &str) {
        println!("{}", message.green().bold());
    }

    pub fn log_stat(value: usize) {
        println!();
        Logger::log_measure(value.to_string().as_str(), "Bytes");
        Logger::log_measure(&((value as f64) / (1024 as f64)).to_string(), "KB");
        println!();
    }

    pub fn log_measure(value: &str, unit: &str) {
        println!("{} {}", Logger::to_value(value).green(), unit.green())
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

    fn to_value(value: &str) -> String {
        let tokens: Vec<&str> = value.split(".").collect();
        if tokens.is_empty() {
            return "0".to_owned();
        }
        if tokens.len() == 1 {
            return Logger::commafy(tokens[0].to_string());
        }
        return format!("{}.{}", Logger::commafy(tokens[0].to_string()), tokens[1]);
    }

    fn commafy(s: String) -> String {
        let result = s
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        result
    }
}
