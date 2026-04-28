use regex::Regex;
use std::sync::{Arc, LazyLock};
use tokio::{join, task::JoinSet};

use crate::{
    critical_path_check::critical_resources::CriticalResources,
    logger::logger::Logger,
    parsers::{
        asset_parser::AssetParser,
        critical_path_parser::CriticalPathParser,
        file_paths::{FilePaths, FileResolutionStrategy},
    },
};

static URL_PARENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(.*)\/"#).unwrap());

pub struct HTMLParser {
    pub html_path: FileResolutionStrategy,
    pub file_resolver: FilePaths,
    pub html_content: String,
}

impl HTMLParser {
    pub fn new(root_html: &FileResolutionStrategy) -> Self {
        let html_content = HTMLParser::resolve_root(root_html);
        let file_resolver = FilePaths::new(&HTMLParser::html_directory(root_html));
        HTMLParser {
            file_resolver,
            html_path: root_html.to_owned(),
            html_content: html_content.to_owned(),
        }
    }

    #[tokio::main]
    pub async fn traverse_linked_resources(self) -> CriticalResources {
        let arc = Arc::new(self);
        let mut task_queue = JoinSet::new();
        task_queue.spawn(arc.clone().create_css_parser());
        task_queue.spawn(arc.clone().create_javascript_parser());
        let [mut css_parser, mut js_parser] = task_queue.join_all().await.try_into().unwrap();
        let (javascript_weight, css_weight) = join!(js_parser.analyze(), css_parser.analyze());
        CriticalResources {
            css_weight,
            javascript_weight,
            html_weight: arc.html_content.len(),
        }
    }

    async fn create_javascript_parser(self: Arc<Self>) -> CriticalPathParser {
        CriticalPathParser::new(
            &self.file_resolver.root_directory,
            Regex::new(r#"(?:^|\W)import\s*(?:(?:(?:[a-zA-Z_$][\w$]*)|(?:\{[^{}]+\})|(?:\*\s+as\s+[a-zA-Z_$][\w$]*)|(?:[a-zA-Z_$][\w$]*\s*,\s*(?:\{[^{}]+\}|(?:\*\s+as\s+[a-zA-Z_$][\w$]*))))\s*from\s*)?['`"]([^'`"]+)['`"]"#).unwrap(),
            AssetParser::create_script_parser(&self.html_path)
                .parse_from(&self.html_content, &self.file_resolver)
                .to_stack(),
        )
    }

    async fn create_css_parser(self: Arc<Self>) -> CriticalPathParser {
        CriticalPathParser::new(
            &self.file_resolver.root_directory,
            Regex::new(r#"(?:^|;|})@import\s*?(?:url\()?['`"]([^'`"]+)['`"](?:\))?"#).unwrap(),
            AssetParser::create_link_parser(&self.html_path)
                .parse_from(&self.html_content, &self.file_resolver)
                .to_stack(),
        )
    }

    fn html_directory(html_path: &FileResolutionStrategy) -> FileResolutionStrategy {
        match html_path {
            FileResolutionStrategy::Http(url) => {
                if url.ends_with(".html")
                    && let Some(result) = URL_PARENT_REGEX.captures(url)
                    && let Some(first_match) = result.get(1)
                {
                    return FileResolutionStrategy::Http(first_match.as_str().to_string());
                }
                html_path.clone()
            }
            FileResolutionStrategy::Local(path) => {
                if let Some(parent_dir) = path.parent() {
                    return FileResolutionStrategy::Local(parent_dir.to_path_buf());
                }
                Logger::panic_with_error("I was unable to determine the HTML's directory");
                panic!();
            }
        }
    }

    fn resolve_root(html_path: &FileResolutionStrategy) -> String {
        if let Some(content) = match html_path {
            FileResolutionStrategy::Http(url) => FilePaths::fetch_resource(url),
            FileResolutionStrategy::Local(path) => FilePaths::read_resource(path),
        } {
            return content;
        }
        Logger::panic_with_error("Failed to parse the root HTML file");
        panic!();
    }
}
