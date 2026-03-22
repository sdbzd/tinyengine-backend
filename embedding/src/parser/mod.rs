pub mod trait_;
pub mod pdf;
pub mod text;

pub use trait_::{DocumentParser, DocumentParserRegistry, ParsedDocument, DocumentMetadata, FileType};
pub use pdf::PdfParser;
pub use text::{TextParser, CodeParser};

use crate::error::Result;

pub async fn parse_file(path: &str) -> Result<ParsedDocument> {
    let mut registry = DocumentParserRegistry::new();
    registry.register(PdfParser::new());
    registry.register(TextParser::new());
    registry.register(CodeParser::new());
    
    registry.parse(path).await
}

pub fn get_supported_extensions() -> Vec<&'static str> {
    vec![
        "pdf",
        "txt",
        "md",
        "markdown",
        "java",
        "py",
        "js",
        "ts",
        "tsx",
        "c",
        "cpp",
        "cc",
        "cxx",
        "rs",
        "go",
        "sql",
        "sh",
        "bash",
        "bat",
        "cmd",
        "html",
        "htm",
        "css",
        "xml",
        "json",
        "yaml",
        "yml",
        "properties",
        "toml",
        "ini",
    ]
}

pub fn is_supported_file(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    get_supported_extensions().iter().any(|ext| {
        path_lower.ends_with(&format!(".{}", ext))
    })
}
