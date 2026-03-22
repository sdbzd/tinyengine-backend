use async_trait::async_trait;
use std::fs;
use encoding_rs::{UTF_8, WINDOWS_1252, GBK, UTF_16LE, UTF_16BE};
use crate::error::{EmbeddingError, Result};
use super::{DocumentParser, ParsedDocument, DocumentMetadata, FileType};
use super::trait_::{CodeLanguage, ConfigFormat};

pub struct TextParser;

impl TextParser {
    pub fn new() -> Self {
        Self
    }

    async fn parse_sync(&self, path: &str) -> Result<ParsedDocument> {
        let bytes = fs::read(path)
            .map_err(|e| EmbeddingError::Io(e))?;

        let (content, encoding) = Self::decode_with_detection(&bytes);

        let file_type = if path.ends_with(".md") {
            FileType::Markdown
        } else {
            FileType::Text
        };

        Ok(ParsedDocument {
            content,
            metadata: DocumentMetadata {
                source: path.to_string(),
                file_type,
                page_count: None,
                encoding: Some(encoding),
            },
        })
    }

    fn decode_with_detection(bytes: &[u8]) -> (String, String) {
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            let content = String::from_utf8_lossy(&bytes[3..]).to_string();
            return (content, "UTF-8".to_string());
        }

        if bytes.starts_with(&[0xFF, 0xFE]) {
            let (content, _, _) = UTF_16LE.decode(bytes);
            return (content.to_string(), "UTF-16LE".to_string());
        }

        if bytes.starts_with(&[0xFE, 0xFF]) {
            let (content, _, _) = UTF_16BE.decode(bytes);
            return (content.to_string(), "UTF-16BE".to_string());
        }

        if let Ok(content) = std::str::from_utf8(bytes) {
            return (content.to_string(), "UTF-8".to_string());
        }

        let (content, _encoding, had_errors) = UTF_8.decode(bytes);
        if !had_errors {
            return (content.to_string(), "UTF-8".to_string());
        }

        let (content, _, _) = WINDOWS_1252.decode(bytes);
        let content_str = content.to_string();
        if !content_str.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
            return (content_str, "Windows-1252".to_string());
        }

        let (content, _, _) = GBK.decode(bytes);
        (content.to_string(), "GBK".to_string())
    }
}

#[async_trait]
impl DocumentParser for TextParser {
    async fn parse(&self, path: &str) -> Result<ParsedDocument> {
        self.parse_sync(path).await
    }

    fn supports(&self, file_type: FileType) -> bool {
        matches!(file_type, FileType::Text | FileType::Markdown)
    }
}

impl Default for TextParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TextParser {
    fn clone(&self) -> Self {
        Self
    }
}

pub struct CodeParser;

impl CodeParser {
    pub fn new() -> Self {
        Self
    }

    async fn parse_sync(&self, path: &str) -> Result<ParsedDocument> {
        let bytes = fs::read(path)
            .map_err(|e| EmbeddingError::Io(e))?;

        let (content, encoding) = TextParser::decode_with_detection(&bytes);

        let file_type = Self::detect_file_type(path);

        Ok(ParsedDocument {
            content,
            metadata: DocumentMetadata {
                source: path.to_string(),
                file_type,
                page_count: None,
                encoding: Some(encoding),
            },
        })
    }

    fn detect_file_type(path: &str) -> FileType {
        let path_lower = path.to_lowercase();
        
        if path_lower.ends_with(".json") {
            return FileType::Code(CodeLanguage::Json);
        }
        if path_lower.ends_with(".xml") {
            return FileType::Code(CodeLanguage::Xml);
        }
        if path_lower.ends_with(".html") || path_lower.ends_with(".htm") {
            return FileType::Code(CodeLanguage::Html);
        }
        if path_lower.ends_with(".css") {
            return FileType::Code(CodeLanguage::Css);
        }
        if path_lower.ends_with(".sql") {
            return FileType::Code(CodeLanguage::Sql);
        }
        if path_lower.ends_with(".yaml") || path_lower.ends_with(".yml") {
            return FileType::Code(CodeLanguage::Yaml);
        }
        if path_lower.ends_with(".properties") {
            return FileType::Config(ConfigFormat::Properties);
        }
        if path_lower.ends_with(".toml") {
            return FileType::Config(ConfigFormat::Toml);
        }
        if path_lower.ends_with(".ini") {
            return FileType::Config(ConfigFormat::Ini);
        }

        FileType::Unknown
    }
}

#[async_trait]
impl DocumentParser for CodeParser {
    async fn parse(&self, path: &str) -> Result<ParsedDocument> {
        self.parse_sync(path).await
    }

    fn supports(&self, file_type: FileType) -> bool {
        matches!(file_type, 
            FileType::Code(_) | 
            FileType::Config(_) |
            FileType::Unknown
        )
    }
}

impl Default for CodeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CodeParser {
    fn clone(&self) -> Self {
        Self
    }
}
