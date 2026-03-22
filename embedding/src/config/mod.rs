use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingConfig {
    #[serde(default = "default_model_path")]
    pub model_path: String,
    #[serde(default = "default_tokenizer_path")]
    pub tokenizer_path: String,
    #[serde(default = "default_dimension")]
    pub dimension: usize,
    #[serde(default = "default_max_length")]
    pub max_length: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

fn default_model_path() -> String {
    "./all-MiniLM-L6-v2/model.onnx".to_string()
}

fn default_tokenizer_path() -> String {
    "./all-MiniLM-L6-v2/tokenizer.json".to_string()
}

fn default_dimension() -> usize {
    384
}

fn default_max_length() -> usize {
    512
}

fn default_batch_size() -> usize {
    32
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model_path: default_model_path(),
            tokenizer_path: default_tokenizer_path(),
            dimension: default_dimension(),
            max_length: default_max_length(),
            batch_size: default_batch_size(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChunkerConfig {
    #[serde(default = "default_chunk_size")]
    pub chunk_size: usize,
    #[serde(default = "default_chunk_overlap")]
    pub chunk_overlap: usize,
}

fn default_chunk_size() -> usize {
    1000
}

fn default_chunk_overlap() -> usize {
    200
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            chunk_size: default_chunk_size(),
            chunk_overlap: default_chunk_overlap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VectorStoreConfig {
    #[serde(default = "default_qdrant_url")]
    pub qdrant_url: String,
    #[serde(default = "default_collection_name")]
    pub collection_name: String,
}

fn default_qdrant_url() -> String {
    "http://localhost:6333".to_string()
}

fn default_collection_name() -> String {
    "tinyengine_documents".to_string()
}

impl Default for VectorStoreConfig {
    fn default() -> Self {
        Self {
            qdrant_url: default_qdrant_url(),
            collection_name: default_collection_name(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_max_results")]
    pub max_results: usize,
    #[serde(default = "default_min_score")]
    pub min_score: f32,
}

fn default_max_results() -> usize {
    10
}

fn default_min_score() -> f32 {
    0.4
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: default_max_results(),
            min_score: default_min_score(),
        }
    }
}
