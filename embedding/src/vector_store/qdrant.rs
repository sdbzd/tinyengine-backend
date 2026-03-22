use async_trait::async_trait;
use crate::error::{EmbeddingError, Result};
use super::{VectorStore, VectorRecord, SearchQuery, SearchResult, CollectionInfo, SearchFilter};

pub struct QdrantStore {
    url: String,
}

impl QdrantStore {
    pub async fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
        })
    }

    pub async fn create_collection_if_not_exists(
        &self,
        _collection_name: &str,
        _dimension: usize,
    ) -> Result<()> {
        Ok(())
    }

    fn record_to_point(record: &VectorRecord) -> serde_json::Value {
        serde_json::json!({
            "id": record.id,
            "vector": record.vector,
            "payload": {
                "content": record.payload.content,
                "source": record.payload.source,
                "collection": record.payload.collection,
            }
        })
    }
}

#[async_trait]
impl VectorStore for QdrantStore {
    async fn upsert(&self, records: Vec<VectorRecord>) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let points: Vec<_> = records.iter().map(Self::record_to_point).collect();
        
        let client = reqwest::Client::new();
        let _response = client
            .put(format!("{}/collections/tinyengine_documents/points", self.url))
            .json(&serde_json::json!({
                "points": points
            }))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        Ok(())
    }

    async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>> {
        if query.vector.is_empty() {
            return Ok(Vec::new());
        }

        let client = reqwest::Client::new();
        
        let response = client
            .post(format!("{}/collections/tinyengine_documents/points/search", self.url))
            .json(&serde_json::json!({
                "vector": query.vector,
                "limit": query.limit,
                "score_threshold": query.score_threshold,
                "with_payload": true
            }))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        let results: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        let search_results: Vec<SearchResult> = results
            .into_iter()
            .map(|point| {
                let id = point["id"].as_str().unwrap_or("").to_string();
                let score = point["score"].as_f64().unwrap_or(0.0) as f32;
                let payload = &point["payload"];
                
                SearchResult {
                    id,
                    score,
                    payload: super::RecordPayload {
                        content: payload["content"].as_str().unwrap_or("").to_string(),
                        source: payload["source"].as_str().unwrap_or("").to_string(),
                        collection: payload["collection"].as_str().unwrap_or("").to_string(),
                        document_set_id: None,
                        created_at: chrono::Utc::now().to_rfc3339(),
                        metadata: None,
                    },
                }
            })
            .collect();

        Ok(search_results)
    }

    async fn delete(&self, ids: Vec<String>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let client = reqwest::Client::new();
        let _response = client
            .post(format!("{}/collections/tinyengine_documents/points/delete", self.url))
            .json(&serde_json::json!({
                "points": ids
            }))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_filter(&self, filter: SearchFilter) -> Result<()> {
        let client = reqwest::Client::new();
        
        let mut filter_json = serde_json::json!({});
        
        if let Some(source) = filter.source {
            filter_json["key"] = serde_json::json!("source");
            filter_json["match"] = serde_json::json!({
                "value": source
            });
        }

        let _response = client
            .post(format!("{}/collections/tinyengine_documents/points/delete", self.url))
            .json(&serde_json::json!({
                "filter": filter_json
            }))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        Ok(())
    }

    async fn get_collection_info(&self, _collection: &str) -> Result<Option<CollectionInfo>> {
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/collections/tinyengine_documents", self.url))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        if response.status().is_success() {
            let info: serde_json::Value = response
                .json()
                .await
                .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;
            
            Ok(Some(CollectionInfo {
                name: "tinyengine_documents".to_string(),
                vectors_count: info["result"]["vectors_count"].as_u64().unwrap_or(0),
                dimension: 384,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_collections(&self) -> Result<Vec<String>> {
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/collections", self.url))
            .send()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| EmbeddingError::Qdrant(e.to_string()))?;

        let collections: Vec<String> = result["result"]["collections"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| c["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(collections)
    }

    async fn collection_exists(&self, collection: &str) -> Result<bool> {
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/collections/{}", self.url, collection))
            .send()
            .await;

        match response {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

impl Clone for QdrantStore {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
        }
    }
}
