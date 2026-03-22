pub mod text_chunker;

pub use text_chunker::{Chunk, SemanticChunker, TextChunker};

use crate::config::ChunkerConfig;

pub fn create_chunker(config: &ChunkerConfig) -> TextChunker {
    TextChunker::from_config(config)
}

pub fn create_semantic_chunker(config: &ChunkerConfig) -> SemanticChunker {
    SemanticChunker::new(config.chunk_size, config.chunk_overlap)
}
