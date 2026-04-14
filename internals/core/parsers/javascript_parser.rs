use std::{collections::VecDeque, path::PathBuf};

use crate::{
    logger::logger::Logger,
    parsers::{
        file_paths::{FilePaths, FileResolutionStrategy},
        traverser::{CriticalPath, Traverser},
    },
};
use futures::executor::block_on;
use swc_common::{FileName, SourceMap};
use swc_common::{SourceFile, sync::Lrc};
use swc_ecma_ast::{ModuleDecl, ModuleItem};
use swc_ecma_parser::{Parser, StringInput, Syntax};

pub struct JavaScriptParser {
    builder: CriticalPath,
    resolution_roots: Vec<PathBuf>,
}

impl JavaScriptParser {
    pub fn new(
        root_directory: &PathBuf,
        paths: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>,
    ) -> Self {
        let builder = JavaScriptParser::create(root_directory, paths);
        JavaScriptParser {
            resolution_roots: [builder.root_directory.to_owned()].to_vec(),
            builder,
        }
    }

    fn to_source_file(
        &self,
        file: &FileResolutionStrategy,
        origin: &FileResolutionStrategy,
    ) -> Option<Lrc<SourceFile>> {
        match file {
            FileResolutionStrategy::Local(path) => {
                let source_map: Lrc<SourceMap> = Default::default();
                if let Ok(source_file) = source_map.load_file(path) {
                    return Some(source_file);
                }
                FilePaths::store_unresolved_path(origin, &FilePaths::hash(file));
                Logger::failed_to_parse_file(&FilePaths::to_string(path));
                None
            }
            FileResolutionStrategy::Http(url) => {
                if let Some(content) = block_on(FilePaths::fetch_resource(url)) {
                    let source_map = SourceMap::default();
                    let file_name = url.to_owned();
                    return Some(
                        source_map.new_source_file(FileName::Custom(file_name).into(), content),
                    );
                }
                FilePaths::store_unresolved_path(origin, url);
                Logger::failed_to_load_file(url);
                None
            }
        }
    }
}

impl Traverser for JavaScriptParser {
    fn traverse(&mut self) -> usize {
        while !self.builder.stack.is_empty() {
            if let Some((file, origin)) = self.builder.stack.pop_back() {
                self.dfs(file, &origin);
            }
        }
        self.builder.weight
    }

    fn dfs(&mut self, file: FileResolutionStrategy, origin: &FileResolutionStrategy) {
        let key = FilePaths::hash(&file);
        if self.builder.visited.contains(&key) {
            return;
        }
        self.builder.visited.insert(key);
        if let Some(source_file) = self.to_source_file(&file, origin) {
            self.builder.weight += source_file.byte_length() as usize;
            let mut parser = Parser::new(
                Syntax::Es(Default::default()),
                StringInput::from(&*source_file),
                None,
            );
            if let Ok(module) = parser.parse_module() {
                for item in module.body {
                    if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item
                        && let Some(reference) = import_decl.src.value.as_str()
                    {
                        match file {
                            FileResolutionStrategy::Http(_) => {
                                let file_system = FilePaths::new(&self.builder.root_directory);
                                if let Some(strategy) =
                                    file_system.resolve_file(reference, &Vec::new())
                                {
                                    self.builder.stack.push_back((strategy, file.clone()));
                                } else {
                                    self.import_reference_error(&file, reference);
                                }
                            }
                            FileResolutionStrategy::Local(ref path) => {
                                let root = path.parent().unwrap_or(path).to_path_buf();
                                let file_system = FilePaths::new(&root);
                                if let Some(strategy) =
                                    file_system.resolve_file(reference, &self.resolution_roots)
                                {
                                    self.builder.stack.push_back((strategy, file.clone()));
                                } else {
                                    self.import_reference_error(&file, reference);
                                }
                            }
                        }
                    }
                }
            } else {
                Logger::failed_to_parse_file(&FilePaths::hash(&file));
            }
        }
    }
}
