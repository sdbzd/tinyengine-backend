pub mod trait_;
pub mod qdrant;

pub use trait_::{VectorStore, VectorRecord, SearchQuery, SearchResult, CollectionInfo, SearchFilter, RecordPayload};
pub use qdrant::QdrantStore;

use crate::error::Result;

pub async fn create_vector_store(url: &str) -> Result<QdrantStore> {
    QdrantStore::new(url).await
}

pub fn supported_vector_stores() -> Vec<&'static str> {
    vec!["qdrant", "chroma"]
}
