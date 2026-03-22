# Tiny Engine Embedding Module

Rust implementation of all-MiniLM-L6-v2 embedding for TinyEngine RAG functionality.

## Features

- **ONNX Model Support**: Load and run all-MiniLM-L6-v2 embeddings
- **HuggingFace Tokenizer**: Full tokenizer support from `tokenizer.json`
- **Document Parsing**: PDF, TXT, MD, code files, and config files
- **Text Chunking**: Configurable chunk size and overlap
- **Vector Storage**: Qdrant client for vector storage and retrieval
- **Semantic Search**: Fast similarity search with filtering

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tiny-engine-embedding = { path = "embedding" }
```

## Quick Start

```rust
use tiny_engine_embedding::{
    create_default_rag_service,
    EmbeddingConfig, ChunkerConfig, VectorStoreConfig, SearchConfig,
    RAGService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rag_service = create_default_rag_service().await?;

    // Embed a single text
    let embedding = rag_service.embed_text("Hello, world!").await?;
    println!("Embedding dimension: {}", embedding.len());

    // Index a document
    rag_service.index_document("/path/to/document.pdf", None).await?;

    // Search
    let results = rag_service.search("your query", Some(5), Some(0.4)).await?;

    Ok(())
}
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MODEL_PATH` | Path to ONNX model | `./all-MiniLM-L6-v2/model.onnx` |
| `TOKENIZER_PATH` | Path to tokenizer | `./all-MiniLM-L6-v2/tokenizer.json` |
| `QDRANT_URL` | Qdrant server URL | `http://localhost:6333` |
| `RAG_CHUNK_SIZE` | Text chunk size | `1000` |
| `RAG_CHUNK_OVERLAP` | Chunk overlap | `200` |
| `RAG_MAX_RESULTS` | Max search results | `10` |
| `RAG_MIN_SCORE` | Min similarity score | `0.4` |

## Supported File Types

- **Documents**: PDF, TXT, MD
- **Code**: Java, Python, JS, TS, C, C++, Rust, Go, SQL, Shell, HTML, CSS
- **Config**: JSON, XML, YAML, Properties, TOML, INI

## Architecture

```
embedding/
├── src/
│   ├── config/          # Configuration structs
│   ├── error/           # Error types and API response
│   ├── model/           # ONNX model and tokenizer
│   ├── parser/          # Document parsers
│   ├── chunker/         # Text chunking
│   ├── vector_store/    # Vector storage
│   └── service/         # Business logic
```

## Examples

```bash
# Run embedding example
cargo run --example embedding_example

# Run search example
cargo run --example search_example -- /path/to/doc.pdf "your query"
```

## License

MIT
