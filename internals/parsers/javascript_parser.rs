use std::{collections::HashSet, fs::read_to_string, path::PathBuf};

use crate::parsers::{
    file_paths::FilePaths,
    traverser::{GraphBuilder, Traverser},
};
use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_ast::{ModuleDecl, ModuleItem};
use swc_ecma_parser::{Parser, StringInput, Syntax};

pub struct JavaScriptParser {
    builder: GraphBuilder,
    root_directory: PathBuf,
}

impl JavaScriptParser {
    pub fn new(root_directory: PathBuf, paths: &HashSet<PathBuf>) -> Self {
        JavaScriptParser {
            root_directory,
            builder: JavaScriptParser::create(paths),
        }
    }
}

impl Traverser for JavaScriptParser {
    fn traverse(&mut self) -> usize {
        while !self.builder.stack.is_empty() {
            if let Some(path) = self.builder.stack.pop_back() {
                self.dfs(path);
            }
        }
        self.builder.weight
    }

    fn dfs(&mut self, path: PathBuf) {
        if self.builder.visited.contains(&path) {
            return;
        }
        self.builder.visited.insert(path.clone());
        let source_map: Lrc<SourceMap> = Default::default();
        if let Ok(source_file) = source_map.load_file(&path) {
            self.builder.weight += read_to_string(&path).unwrap_or("".to_string()).len();
            let mut parser = Parser::new(
                Syntax::Es(Default::default()),
                StringInput::from(&*source_file),
                None,
            );
            if let Ok(module) = parser.parse_module() {
                for item in module.body {
                    if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item
                        && let Some(source) = import_decl.src.value.as_str()
                    {
                        if source.starts_with("http") {
                            if let Some(source_path) = FilePaths::new(self.root_directory.clone())
                                .to_file_system_path(source)
                            {
                                self.builder.stack.push_back(source_path);
                            } else {
                                FilePaths::store_unresolved_path(&path, source);
                            }
                        } else if let Some(current_module_path) = &path.parent()
                            && let Some(source_path) =
                                FilePaths::new(current_module_path.to_path_buf())
                                    .to_file_system_path(source)
                        {
                            self.builder.stack.push_back(source_path);
                        }
                    }
                }
            }
        }
    }
}
