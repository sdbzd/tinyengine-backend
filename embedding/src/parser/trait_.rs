use async_trait::async_trait;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub content: String,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub source: String,
    pub file_type: FileType,
    pub page_count: Option<usize>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Pdf,
    Text,
    Markdown,
    Code(CodeLanguage),
    Config(ConfigFormat),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeLanguage {
    Java,
    Python,
    JavaScript,
    TypeScript,
    C,
    Cpp,
    Rust,
    Go,
    Sql,
    Shell,
    Html,
    Css,
    Xml,
    Json,
    Yaml,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Properties,
    Toml,
    Ini,
    Other,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "pdf" => FileType::Pdf,
            "txt" => FileType::Text,
            "md" | "markdown" => FileType::Markdown,
            "java" => FileType::Code(CodeLanguage::Java),
            "py" => FileType::Code(CodeLanguage::Python),
            "js" => FileType::Code(CodeLanguage::JavaScript),
            "ts" | "tsx" => FileType::Code(CodeLanguage::TypeScript),
            "c" => FileType::Code(CodeLanguage::C),
            "cpp" | "cc" | "cxx" => FileType::Code(CodeLanguage::Cpp),
            "rs" => FileType::Code(CodeLanguage::Rust),
            "go" => FileType::Code(CodeLanguage::Go),
            "sql" => FileType::Code(CodeLanguage::Sql),
            "sh" | "bash" | "bat" | "cmd" => FileType::Code(CodeLanguage::Shell),
            "html" | "htm" => FileType::Code(CodeLanguage::Html),
            "css" => FileType::Code(CodeLanguage::Css),
            "xml" => FileType::Code(CodeLanguage::Xml),
            "json" => FileType::Code(CodeLanguage::Json),
            "yaml" | "yml" => FileType::Code(CodeLanguage::Yaml),
            "properties" => FileType::Config(ConfigFormat::Properties),
            "toml" => FileType::Config(ConfigFormat::Toml),
            "ini" => FileType::Config(ConfigFormat::Ini),
            _ => FileType::Unknown,
        }
    }
}

#[async_trait]
pub trait DocumentParser: Send + Sync {
    async fn parse(&self, path: &str) -> Result<ParsedDocument>;
    fn supports(&self, file_type: FileType) -> bool;
}

pub struct DocumentParserRegistry {
    parsers: Vec<Box<dyn DocumentParser>>,
}

impl DocumentParserRegistry {
    pub fn new() -> Self {
        Self { parsers: Vec::new() }
    }

    pub fn register(&mut self, parser: impl DocumentParser + 'static) {
        self.parsers.push(Box::new(parser));
    }

    pub async fn parse(&self, path: &str) -> Result<ParsedDocument> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let file_type = FileType::from_extension(ext);

        for parser in &self.parsers {
            if parser.supports(file_type) {
                return parser.parse(path).await;
            }
        }

        crate::parser::text::TextParser::new().parse(path).await
    }
}

impl Default for DocumentParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}
