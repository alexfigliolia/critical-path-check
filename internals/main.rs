use crate::argv::argv::{Argv, ArgvOption, ArgvType};
use crate::critical_path_check::critical_path_check::CriticalPathCheck;

mod argv;
mod critical_path_check;
mod logger;
mod parsers;

fn main() {
    let parser = Argv::new(
        [ArgvOption {
            value_type: ArgvType::Boolean,
            name: "json",
            short: Some("j"),
            multiple: None,
        }],
        None,
    );
    let empty = String::from("");
    let path = parser.positionals.last().unwrap_or(&empty);
    if parser.has("json") {
        return CriticalPathCheck::new(path).as_json();
    }
    CriticalPathCheck::new(path).run_cli();
}
