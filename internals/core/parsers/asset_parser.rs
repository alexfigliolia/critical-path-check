use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use regex::Regex;

use crate::{
    logger::logger::Logger,
    parsers::file_paths::FilePaths,
    visitor::{explored_paths::ExploredPaths, file_resolution_strategy::FileResolutionStrategy},
};

#[derive(Debug)]
pub struct AssetParser {
    parser: Regex,
    origin: FileResolutionStrategy,
    paths: Arc<Mutex<ExploredPaths>>,
    linked_assets: HashSet<FileResolutionStrategy>,
}

impl AssetParser {
    pub fn new(
        origin: &FileResolutionStrategy,
        parser: Regex,
        paths: Arc<Mutex<ExploredPaths>>,
    ) -> Self {
        Self {
            paths,
            parser,
            origin: origin.to_owned(),
            linked_assets: HashSet::new(),
        }
    }

    pub fn create_link_parser(
        html_path: &FileResolutionStrategy,
        paths: Arc<Mutex<ExploredPaths>>,
    ) -> AssetParser {
        AssetParser::new(
            html_path,
            Regex::new(r#"<link.*?rel=['"]stylesheet["'].*?href=['"]([^'"]+)['"].*?>"#).unwrap(),
            paths,
        )
    }

    pub fn create_script_parser(
        html_path: &FileResolutionStrategy,
        paths: Arc<Mutex<ExploredPaths>>,
    ) -> AssetParser {
        AssetParser::new(
            html_path,
            Regex::new(r#"<script.*?src=['"]([^'"]+)['"].*?>"#).unwrap(),
            paths,
        )
    }

    pub fn parse_from(mut self, content: &str, resolver: &FilePaths) -> Self {
        let captures = self.parser.captures_iter(content);
        for capture in captures {
            if let Some(group) = capture.get(1) {
                let group_str = group.as_str();
                let mut paths = self.paths.lock().unwrap();
                if let Some(resolution) = resolver.resolve_file(group_str, &Vec::new()) {
                    self.linked_assets.insert(resolution);
                } else {
                    paths.store_unresolved_path(&self.origin, group_str);
                    Logger::path_error(group_str);
                }
            }
        }
        self
    }

    pub fn to_stack(&self) -> Vec<(FileResolutionStrategy, FileResolutionStrategy)> {
        Vec::from_iter(
            self.linked_assets
                .iter()
                .map(|file| (file.to_owned(), self.origin.clone())),
        )
    }
}
