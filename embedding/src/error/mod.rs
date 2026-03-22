use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Model loading failed: {0}")]
    ModelLoad(String),

    #[error("Tokenizer loading failed: {0}")]
    TokenizerLoad(String),

    #[error("Tokenization failed: {0}")]
    Tokenization(String),

    #[error("Inference failed: {0}")]
    Inference(String),

    #[error("Vector store error: {0}")]
    VectorStore(String),

    #[error("Document parsing failed: {0}")]
    Parsing(String),

    #[error("Chunking failed: {0}")]
    Chunking(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Qdrant error: {0}")]
    Qdrant(String),

    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, EmbeddingError>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: "COMMON.success".to_string(),
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: &str, msg: &str) -> Self {
        Self {
            code: code.to_string(),
            msg: msg.to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingMatchDto {
    pub embedding_id: String,
    pub create_time: String,
    pub score: f32,
    pub content: String,
    pub metadata: EmbeddingMetadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingMetadata {
    pub collection: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_set_id: Option<String>,
}

impl EmbeddingMatchDto {
    pub fn new(
        embedding_id: String,
        content: String,
        score: f32,
        collection: String,
        source: String,
    ) -> Self {
        Self {
            embedding_id,
            create_time: chrono::Utc::now().to_rfc3339(),
            score,
            content,
            metadata: EmbeddingMetadata {
                collection: collection.clone(),
                source: source.clone(),
                document_set_id: None,
            },
        }
    }
}
