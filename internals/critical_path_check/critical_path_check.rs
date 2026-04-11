use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{dependency_graph::dependency_graph::DependencyGraph, logger::logger::Logger};

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
        self.build_graph().javascript_weight < bytes
    }

    pub fn assert_css(&self, bytes: usize) -> bool {
        self.build_graph().css_weight < bytes
    }

    pub fn assert_uncategorized(&self, bytes: usize) -> bool {
        self.build_graph().uncategorized_weight < bytes
    }

    pub fn measure(&self) -> usize {
        let graph = self.build_graph();
        graph.total_weight()
    }

    fn build_graph(&self) -> DependencyGraph {
        let mut graph = DependencyGraph::new(&self.root_html);
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
        path.to_path_buf()
    }
}
