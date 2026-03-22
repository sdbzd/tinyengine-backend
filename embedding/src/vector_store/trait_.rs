use async_trait::async_trait;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct VectorRecord {
    pub id: String,
    pub vector: Vec<f32>,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecordPayload {
    pub content: String,
    pub source: String,
    pub collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_set_id: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchQuery {
    pub vector: Vec<f32>,
    pub limit: usize,
    pub score_threshold: Option<f32>,
    pub filter: Option<SearchFilter>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchFilter {
    pub collection: Option<String>,
    pub source: Option<String>,
    pub document_set_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    pub vectors_count: u64,
    pub dimension: usize,
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, records: Vec<VectorRecord>) -> Result<()>;
    async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>>;
    async fn delete(&self, ids: Vec<String>) -> Result<()>;
    async fn delete_by_filter(&self, filter: SearchFilter) -> Result<()>;
    async fn get_collection_info(&self, collection: &str) -> Result<Option<CollectionInfo>>;
    async fn list_collections(&self) -> Result<Vec<String>>;
    async fn collection_exists(&self, collection: &str) -> Result<bool>;
}
