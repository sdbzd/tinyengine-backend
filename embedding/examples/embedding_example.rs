use tiny_engine_embedding::{
    EmbeddingConfig, ChunkerConfig, VectorStoreConfig, SearchConfig,
    RAGService,
};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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

    println!("Initializing RAG service...");
    let rag_service = RAGService::new(
        embedding_config,
        chunker_config,
        vector_store_config,
        search_config,
    ).await?;

    println!("RAG service initialized successfully!");
    println!("Embedding dimension: {}", rag_service.dimension());

    let test_text = "This is a test document for embedding.";
    println!("\nEmbedding text: \"{}\"", test_text);
    
    let embedding = rag_service.embed_text(test_text).await?;
    println!("Embedding vector dimension: {}", embedding.len());
    println!("First 5 values: {:?}", &embedding[..5.min(embedding.len())]);

    match rag_service.search("test query", Some(5), Some(0.3)).await {
        Ok(search_results) => {
            println!("\nSearch returned {} results", search_results.len());
        }
        Err(e) => {
            println!("\nSearch skipped (Qdrant not available): {}", e);
        }
    }

    println!("\nExample completed successfully!");
    Ok(())
}
