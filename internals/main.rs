use std::env::args;

use crate::{
    critical_path_check::critical_path_check::CriticalPathCheck, logger::logger::Logger,
    parsers::file_paths::FilePaths,
};

mod critical_path_check;
mod logger;
mod parsers;

#[tokio::main]
async fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() < 2 {
        Logger::panic_with_error("Please specify an absolute path to a directory");
    }
    let path = &argv[1];
    let graph = CriticalPathCheck::new(path).run();
    FilePaths::log_unresolved();
    graph.log_stats();
}
