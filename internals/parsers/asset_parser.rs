use std::collections::HashSet;

use regex::Regex;

use crate::parsers::file_paths::{FilePaths, FileResolutionStrategy};

#[derive(Debug)]
pub struct AssetParser {
    parser: Regex,
    origin: FileResolutionStrategy,
    linked_assets: HashSet<FileResolutionStrategy>,
}

impl AssetParser {
    pub fn new(origin: &FileResolutionStrategy, parser: Regex) -> Self {
        Self {
            parser,
            origin: origin.to_owned(),
            linked_assets: HashSet::new(),
        }
    }

    pub fn create_link_parser(html_path: &FileResolutionStrategy) -> AssetParser {
        AssetParser::new(
            html_path,
            Regex::new(r#"<link.*?rel=['"]stylesheet["'].*?href=['"]([^'"]+)['"].*?>"#).unwrap(),
        )
    }

    pub fn create_script_parser(html_path: &FileResolutionStrategy) -> AssetParser {
        AssetParser::new(
            html_path,
            Regex::new(r#"<script.*?src=['"]([^'"]+)['"].*?>"#).unwrap(),
        )
    }

    pub fn parse_from(mut self, content: &str, resolver: &FilePaths) -> Self {
        let captures = self.parser.captures_iter(content);
        for capture in captures {
            if let Some(group) = capture.get(1) {
                if let Some(resolution) = resolver.resolve_file(group.as_str(), &Vec::new()) {
                    self.linked_assets.insert(resolution);
                } else {
                    FilePaths::store_unresolved_path(&self.origin, group.as_str());
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
