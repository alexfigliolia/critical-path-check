use clap::Parser;

use crate::critical_path_check::critical_path_check::CriticalPathCheck;

mod critical_path_check;
mod logger;
mod parsers;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    json: bool,
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or("".to_string());
    if args.json {
        return CriticalPathCheck::new(&path).as_json();
    }
    CriticalPathCheck::new(&path).run_cli();
}
