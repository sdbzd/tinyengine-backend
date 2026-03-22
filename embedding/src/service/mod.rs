use std::sync::Arc;
use crate::error::Result;
use crate::model::{EmbeddingModel, HuggingFaceTokenizer, EmbeddingService};
use crate::parser::{DocumentParserRegistry, PdfParser, TextParser, CodeParser};
use crate::chunker::TextChunker;
use crate::vector_store::{VectorStore, VectorRecord, RecordPayload, SearchQuery, SearchFilter, QdrantStore};
use crate::config::{EmbeddingConfig, ChunkerConfig, VectorStoreConfig, SearchConfig};

#[derive(Clone)]
pub struct RAGService {
    embedding_service: Arc<EmbeddingService>,
    tokenizer: Arc<HuggingFaceTokenizer>,
    chunker: Arc<TextChunker>,
    vector_store: Arc<QdrantStore>,
    config: ServiceConfig,
}

#[derive(Clone)]
pub struct ServiceConfig {
    pub embedding: EmbeddingConfig,
    pub chunker: ChunkerConfig,
    pub vector_store: VectorStoreConfig,
    pub search: SearchConfig,
}

impl RAGService {
    pub async fn new(
        embedding_config: EmbeddingConfig,
        chunker_config: ChunkerConfig,
        vector_store_config: VectorStoreConfig,
        search_config: SearchConfig,
    ) -> Result<Self> {
        let tokenizer = HuggingFaceTokenizer::from_file(&embedding_config.tokenizer_path)?;
        
        let model = EmbeddingModel::from_file(&embedding_config.model_path)?;

        let embedding_service = Arc::new(EmbeddingService::new(model));
        let tokenizer = Arc::new(tokenizer);
        let chunker = Arc::new(TextChunker::from_config(&chunker_config));
        let vector_store = Arc::new(QdrantStore::new(&vector_store_config.qdrant_url).await?);

        vector_store.create_collection_if_not_exists(
            &vector_store_config.collection_name,
            embedding_config.dimension,
        ).await?;

        Ok(Self {
            embedding_service,
            tokenizer,
            chunker,
            vector_store,
            config: ServiceConfig {
                embedding: embedding_config,
                chunker: chunker_config,
                vector_store: vector_store_config,
                search: search_config,
            },
        })
    }

    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let encoding = self.tokenizer.encode(text)?;
        self.embedding_service.embed_text(text, &encoding).await
    }

    pub async fn embed_texts(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let encodings: Result<Vec<_>> = texts
            .iter()
            .map(|t| self.tokenizer.encode(t))
            .collect();
        
        self.embedding_service.embed_texts(texts, &encodings?).await
    }

    pub async fn parse_and_chunk_document(&self, path: &str) -> Result<Vec<(String, String)>> {
        let mut registry = DocumentParserRegistry::new();
        registry.register(PdfParser::new());
        registry.register(TextParser::new());
        registry.register(CodeParser::new());

        let doc = registry.parse(path).await?;
        let chunks = self.chunker.chunk(&doc.content);

        Ok(chunks
            .into_iter()
            .map(|c| (c.content, path.to_string()))
            .collect())
    }

    pub async fn index_document(&self, path: &str, collection: Option<String>) -> Result<usize> {
        let chunks = self.parse_and_chunk_document(path).await?;
        let collection = collection.unwrap_or_else(|| self.config.vector_store.collection_name.clone());
        
        if chunks.is_empty() {
            return Ok(0);
        }

        let texts: Vec<String> = chunks.iter().map(|(c, _)| c.clone()).collect();
        let embeddings = self.embed_texts(&texts).await?;

        let records: Vec<VectorRecord> = chunks
            .iter()
            .zip(embeddings.iter())
            .enumerate()
            .map(|(idx, ((content, source), embedding))| {
                VectorRecord {
                    id: format!("{}_{}", source, idx),
                    vector: embedding.clone(),
                    payload: RecordPayload {
                        content: content.clone(),
                        source: source.clone(),
                        collection: collection.clone(),
                        document_set_id: None,
                        created_at: chrono::Utc::now().to_rfc3339(),
                        metadata: None,
                    },
                }
            })
            .collect();

        self.vector_store.upsert(records).await?;

        Ok(chunks.len())
    }

    pub async fn search(&self, query: &str, limit: Option<usize>, min_score: Option<f32>) -> Result<Vec<crate::error::EmbeddingMatchDto>> {
        let embedding = self.embed_text(query).await?;
        
        let search_query = SearchQuery {
            vector: embedding,
            limit: limit.unwrap_or(self.config.search.max_results),
            score_threshold: min_score.or(Some(self.config.search.min_score)),
            filter: None,
        };

        let results = self.vector_store.search(search_query).await?;

        Ok(results
            .into_iter()
            .map(|r| crate::error::EmbeddingMatchDto {
                embedding_id: r.id,
                create_time: r.payload.created_at,
                score: r.score,
                content: r.payload.content,
                metadata: crate::error::EmbeddingMetadata {
                    collection: r.payload.collection,
                    source: r.payload.source,
                    document_set_id: r.payload.document_set_id,
                },
            })
            .collect())
    }

    pub async fn delete_by_source(&self, source: &str) -> Result<()> {
        let filter = SearchFilter {
            collection: None,
            source: Some(source.to_string()),
            document_set_id: None,
        };

        self.vector_store.delete_by_filter(filter).await
    }

    pub async fn list_collections(&self) -> Result<Vec<String>> {
        self.vector_store.list_collections().await
    }

    pub async fn get_collection_info(&self, collection: &str) -> Result<Option<crate::vector_store::CollectionInfo>> {
        self.vector_store.get_collection_info(collection).await
    }

    pub fn dimension(&self) -> usize {
        self.config.embedding.dimension
    }
}
