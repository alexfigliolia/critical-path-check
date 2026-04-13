use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::{
    critical_path_check::critical_resources::CriticalResources, logger::logger::Logger,
    parsers::file_paths::FilePaths,
};

pub struct CriticalPathCheck {
    root_html: PathBuf,
}

impl CriticalPathCheck {
    pub fn new(root_html: &str) -> Self {
        CriticalPathCheck {
            root_html: CriticalPathCheck::validate_path(root_html),
        }
    }

    pub fn assert(&self, bytes: usize) -> bool {
        self.measure() < bytes
    }

    pub fn assert_javascript(&self, bytes: usize) -> bool {
        self.run().javascript_weight < bytes
    }

    pub fn assert_css(&self, bytes: usize) -> bool {
        self.run().css_weight < bytes
    }

    pub fn assert_uncategorized(&self, bytes: usize) -> bool {
        self.run().uncategorized_weight < bytes
    }

    pub fn measure(&self) -> usize {
        let graph = self.run();
        graph.total_weight()
    }

    pub fn run(&self) -> CriticalResources {
        let mut graph = CriticalResources::new(&self.root_html);
        graph.build();
        graph
    }

    fn validate_path(root_html: &str) -> PathBuf {
        let path = Path::new(root_html);
        if !path.is_absolute() {
            Logger::panic_with_error(
                "Your input path must be an absolute path to your root HTML file",
            );
        }
        if !path.exists() {
            Logger::panic_with_error("The specified input path does not exist");
        }
        if path.is_dir() || path.extension().unwrap_or(OsStr::new("")) != "html" {
            Logger::panic_with_error("The specified input does not point to an html file");
        }
        let path_buf = path.to_path_buf();
        Logger::info(
            format!(
                "Analyzing {}",
                FilePaths::to_string(&path_buf).bright_blue()
            )
            .as_str(),
        );
        path_buf
    }
}
