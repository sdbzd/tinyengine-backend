use async_trait::async_trait;
use lopdf::Document;
use crate::error::{EmbeddingError, Result};
use super::{DocumentParser, ParsedDocument, DocumentMetadata, FileType};

pub struct PdfParser;

impl PdfParser {
    pub fn new() -> Self {
        Self
    }

    async fn parse_sync(&self, path: &str) -> Result<ParsedDocument> {
        let doc = Document::load(path)
            .map_err(|e| EmbeddingError::Parsing(format!("Failed to load PDF: {}", e)))?;

        let page_count = doc.get_pages().len();
        let mut content = String::new();

        let page_ids: Vec<u32> = doc.page_iter().map(|(id, _)| id).collect();
        
        for page_id in page_ids {
            if let Ok(text) = doc.extract_text(&[page_id]) {
                content.push_str(&text);
                content.push('\n');
            }
        }

        let content = Self::clean_text(&content);

        Ok(ParsedDocument {
            content,
            metadata: DocumentMetadata {
                source: path.to_string(),
                file_type: FileType::Pdf,
                page_count: Some(page_count),
                encoding: None,
            },
        })
    }

    fn clean_text(text: &str) -> String {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[async_trait]
impl DocumentParser for PdfParser {
    async fn parse(&self, path: &str) -> Result<ParsedDocument> {
        self.parse_sync(path).await
    }

    fn supports(&self, file_type: FileType) -> bool {
        matches!(file_type, FileType::Pdf)
    }
}

impl Default for PdfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PdfParser {
    fn clone(&self) -> Self {
        Self
    }
}
