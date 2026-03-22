use tiny_engine_embedding::{
    EmbeddingConfig, ChunkerConfig, VectorStoreConfig, SearchConfig,
    RAGService,
};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let rag_service = create_rag_service().await?;

    let doc_path = std::env::args().nth(1)
        .expect("Usage: search_example <document_path> [query]");

    let query = std::env::args().nth(2)
        .unwrap_or_else(|| "What is this document about?".to_string());

    println!("Indexing document: {}", doc_path);
    let chunk_count = rag_service.index_document(&doc_path, None).await?;
    println!("Indexed {} chunks", chunk_count);

    println!("\nSearching for: \"{}\"", query);
    let results = rag_service.search(&query, Some(5), Some(0.4)).await?;

    if results.is_empty() {
        println!("No results found.");
    } else {
        println!("\nSearch Results:");
        println!("{}", "=".repeat(80));
        for (i, result) in results.iter().enumerate() {
            println!("\n[{}] Score: {:.4}", i + 1, result.score);
            println!("Source: {}", result.metadata.source);
            let content_preview = if result.content.len() > 200 {
                format!("{}...", &result.content[..200])
            } else {
                result.content.clone()
            };
            println!("Content: {}", content_preview);
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("Example completed!");
    Ok(())
}

async fn create_rag_service() -> std::result::Result<RAGService, Box<dyn std::error::Error>> {
    let embedding_config = EmbeddingConfig {
        model_path: std::env::var("MODEL_PATH")
            .unwrap_or_else(|_| "../all-MiniLM-L6-v2/model.onnx".to_string()),
        tokenizer_path: std::env::var("TOKENIZER_PATH")
            .unwrap_or_else(|_| "../all-MiniLM-L6-v2/tokenizer.json".to_string()),
        dimension: 384,
        max_length: 512,
        batch_size: 32,
    };

    let chunker_config = ChunkerConfig {
        chunk_size: 1000,
        chunk_overlap: 200,
    };

    let vector_store_config = VectorStoreConfig {
        qdrant_url: std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6333".to_string()),
        collection_name: "tinyengine_documents".to_string(),
    };

    let search_config = SearchConfig {
        max_results: 10,
        min_score: 0.4,
    };

    RAGService::new(
        embedding_config,
        chunker_config,
        vector_store_config,
        search_config,
    ).await.map_err(|e| e.into())
}
