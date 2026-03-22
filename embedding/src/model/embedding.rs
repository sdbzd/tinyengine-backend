use crate::error::Result;
use crate::model::onnx_model::OnnxModel;
use crate::model::tokenizer::Encoding;
use std::sync::Arc;

pub struct EmbeddingModel {
    onnx_model: OnnxModel,
    dimension: usize,
}

impl EmbeddingModel {
    pub fn from_file(model_path: &str) -> Result<Self> {
        let onnx_model = OnnxModel::from_file(model_path)?;
        let dimension = onnx_model.dimension();

        Ok(Self {
            onnx_model,
            dimension,
        })
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn embed(&mut self, encoding: &Encoding) -> Result<Vec<f32>> {
        self.onnx_model.infer(&encoding.ids, &encoding.attention_mask)
    }

    pub fn embed_batch(&mut self, encodings: &[Encoding]) -> Result<Vec<Vec<f32>>> {
        encodings.iter().map(|e| self.embed(e)).collect()
    }
}

pub struct EmbeddingService {
    model: Arc<tokio::sync::Mutex<EmbeddingModel>>,
}

impl EmbeddingService {
    pub fn new(model: EmbeddingModel) -> Self {
        Self {
            model: Arc::new(tokio::sync::Mutex::new(model)),
        }
    }

    pub async fn embed_text(&self, _text: &str, encoding: &Encoding) -> Result<Vec<f32>> {
        let mut model = self.model.lock().await;
        model.embed(encoding)
    }

    pub async fn embed_texts(&self, _texts: &[String], encodings: &[Encoding]) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().await;
        model.embed_batch(encodings)
    }

    pub async fn dimension_async(&self) -> usize {
        let model = self.model.lock().await;
        model.dimension()
    }
}
