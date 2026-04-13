// use std::collections::VecDeque;
// use std::{fs::read_to_string, path::PathBuf};

// use lightningcss::rules::CssRule;
// use lightningcss::stylesheet::{ParserOptions, StyleSheet};

// use crate::parsers::file_paths::FileResolutionStrategy;
// use crate::parsers::traverser::{CriticalPath, Traverser};

// pub struct CSSParser {
//     builder: CriticalPath,
// }

// impl CSSParser {
//     pub fn new(root_directory: &PathBuf, paths: VecDeque<(FileResolutionStrategy, FileResolutionStrategy)>) -> Self {
//         CSSParser {
//             builder: CSSParser::create(root_directory, paths),
//         }
//     }
// }

// impl Traverser for CSSParser {
//     fn traverse(&mut self) -> usize {
//         while !self.builder.stack.is_empty() {
//             if let Some((path_string, path_buf)) = self.builder.stack.pop_back() {
//                 self.dfs(path_string, path_buf);
//             }
//         }
//         self.builder.weight
//     }

//     fn dfs(&mut self, path_string: String, path: PathBuf) {
//         if self.builder.visited.contains(&path_string) {
//             return;
//         }
//         println!("{}", path_string.clone());
//         self.builder.visited.insert(path_string.clone());
//         if path.exists() {
//             if let Ok(content) = read_to_string(&path)
//                 && let Ok(stylesheet) = StyleSheet::parse(&content, ParserOptions::default())
//             {
//                 for rule in stylesheet.rules.0 {
//                     if let CssRule::Import(import) = rule {
//                         self.builder.stack.push_back(self.resolve_import(
//                             &self.root_directory,
//                             &path,
//                             &import.url,
//                         ));
//                     }
//                 }
//                 return;
//             }
//         } else if path_string.starts_with("http") {
//             // TODO - resolve over HTTP
//         }
//         // TODO - store unresolved paths
//         FilePaths::store_unresolved_path(root, path);
//     }
// }
