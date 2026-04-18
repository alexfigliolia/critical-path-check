use std::process::exit;

use colored::{ColoredString, Colorize};

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        eprintln!("{}{}", Logger::info_prefix(), message);
    }

    pub fn error(message: &str) {
        eprintln!("{}{}", Logger::error_prefix(), message);
    }

    pub fn path_error(path: &str) {
        Logger::error(format!("Failed to resolve module at: {}", path.bright_blue()).as_str());
    }

    pub fn traversal_error(path: &str) {
        Logger::error(format!("Failed to traverse file: {}", path.bright_blue()).as_str());
    }

    pub fn failed_to_parse_file(path: &str) {
        Logger::error(format!("Failed to parse file: {}", path.bright_blue()).as_str());
    }

    pub fn failed_to_load_file(path: &str) {
        Logger::error(format!("Failed to load remote file: {}", path.bright_blue()).as_str());
    }

    pub fn file_issue() {
        eprintln!();
        eprintln!(
            "{}{}",
            Logger::indent(None),
            "https://github.com/alexfigliolia/critical-path-check/issues".bright_blue()
        );
        eprintln!();
    }

    pub fn green(message: &str) {
        println!("{}", message.green().bold());
    }

    pub fn log_stat(value: usize) {
        println!();
        Logger::log_measure(value.to_string().as_str(), "Bytes");
        Logger::log_measure(&((value as f64) / 1024_f64).to_string(), "KB");
        println!();
    }

    pub fn log_measure(value: &str, unit: &str) {
        println!(
            "{}{} {}",
            Logger::indent(None),
            Logger::to_value(value).cyan(),
            unit.cyan()
        )
    }

    pub fn panic_with_error(message: &str) {
        Logger::error(message);
        exit(1);
    }

    pub fn indent(n: Option<usize>) -> String {
        " ".repeat(n.unwrap_or(3))
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
        format!("{}.{}", Logger::commafy(tokens[0].to_string()), tokens[1])
    }

    fn commafy(s: String) -> String {
        s.as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join(",")
    }
}
