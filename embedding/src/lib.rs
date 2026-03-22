pub mod config;
pub mod error;
pub mod model;
pub mod parser;
pub mod chunker;
pub mod vector_store;
pub mod service;

pub use config::{EmbeddingConfig, ChunkerConfig, VectorStoreConfig, SearchConfig};
pub use error::{EmbeddingError, Result, ApiResponse, EmbeddingMatchDto, EmbeddingMetadata};
pub use model::{EmbeddingModel, HuggingFaceTokenizer, EmbeddingService};
pub use parser::{ParsedDocument, DocumentMetadata, FileType, parse_file, is_supported_file, get_supported_extensions};
pub use chunker::{TextChunker, SemanticChunker, Chunk, create_chunker, create_semantic_chunker};
pub use vector_store::{VectorStore, VectorRecord, SearchQuery, SearchResult, CollectionInfo, SearchFilter, QdrantStore};
pub use service::{RAGService, ServiceConfig};

pub async fn create_default_rag_service() -> Result<RAGService> {
    let embedding_config = EmbeddingConfig::default();
    let chunker_config = ChunkerConfig::default();
    let vector_store_config = VectorStoreConfig::default();
    let search_config = SearchConfig::default();

    RAGService::new(
        embedding_config,
        chunker_config,
        vector_store_config,
        search_config,
    ).await
}

pub async fn create_rag_service_from_env() -> Result<RAGService> {
    let embedding_config = EmbeddingConfig {
        model_path: std::env::var("MODEL_PATH").unwrap_or_else(|_| "./all-MiniLM-L6-v2/model.onnx".to_string()),
        tokenizer_path: std::env::var("TOKENIZER_PATH").unwrap_or_else(|_| "./all-MiniLM-L6-v2/tokenizer.json".to_string()),
        dimension: std::env::var("EMBEDDING_DIMENSION")
            .unwrap_or_else(|_| "384".to_string())
            .parse()
            .unwrap_or(384),
        max_length: std::env::var("MAX_LENGTH")
            .unwrap_or_else(|_| "512".to_string())
            .parse()
            .unwrap_or(512),
        batch_size: std::env::var("BATCH_SIZE")
            .unwrap_or_else(|_| "32".to_string())
            .parse()
            .unwrap_or(32),
    };

    let chunker_config = ChunkerConfig {
        chunk_size: std::env::var("RAG_CHUNK_SIZE")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .unwrap_or(1000),
        chunk_overlap: std::env::var("RAG_CHUNK_OVERLAP")
            .unwrap_or_else(|_| "200".to_string())
            .parse()
            .unwrap_or(200),
    };

    let vector_store_config = VectorStoreConfig {
        qdrant_url: std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6333".to_string()),
        collection_name: std::env::var("COLLECTION_NAME")
            .unwrap_or_else(|_| "tinyengine_documents".to_string()),
    };

    let search_config = SearchConfig {
        max_results: std::env::var("RAG_MAX_RESULTS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
        min_score: std::env::var("RAG_MIN_SCORE")
            .unwrap_or_else(|_| "0.4".to_string())
            .parse()
            .unwrap_or(0.4),
    };

    RAGService::new(
        embedding_config,
        chunker_config,
        vector_store_config,
        search_config,
    ).await
}
