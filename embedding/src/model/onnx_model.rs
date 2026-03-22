use crate::error::{EmbeddingError, Result};
use ndarray::Array2;
use tract_onnx::prelude::*;

pub struct OnnxModel {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, TypedModel>,
    input_name: String,
    dimension: usize,
}

impl std::fmt::Debug for OnnxModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnnxModel")
            .field("input_name", &self.input_name)
            .field("dimension", &self.dimension)
            .finish()
    }
}

impl OnnxModel {
    pub fn from_file(model_path: &str) -> Result<Self> {
        let model_path = std::path::Path::new(model_path);
        if !model_path.exists() {
            return Err(EmbeddingError::ModelLoad(format!(
                "Model file not found: {}",
                model_path.display()
            )));
        }

        let model = tract_onnx::onnx()
            .model_for_path(model_path)
            .map_err(|e| EmbeddingError::ModelLoad(format!("Failed to load model: {}", e)))?;

        let input_name = model
            .input_fact(0)
            .ok()
            .map(|_f| "input".to_string())
            .unwrap_or_else(|| "input".to_string());

        let model = model
            .into_optimized()
            .map_err(|e| EmbeddingError::ModelLoad(format!("Failed to optimize model: {}", e)))?
            .into_runnable()
            .map_err(|e| EmbeddingError::ModelLoad(format!("Failed to make runnable: {}", e)))?;

        let dimension = 384;

        Ok(Self {
            model,
            input_name,
            dimension,
        })
    }

    pub fn input_name(&self) -> &str {
        &self.input_name
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn infer(&mut self, token_ids: &[u32], attention_mask: &[u32]) -> Result<Vec<f32>> {
        if token_ids.is_empty() {
            return Err(EmbeddingError::Inference(
                "Token IDs cannot be empty".to_string(),
            ));
        }

        let seq_len = token_ids.len();

        let input_ids = Tensor::from(
            Array2::<i64>::from_shape_vec(
                (1, seq_len),
                token_ids.iter().map(|&id| id as i64).collect(),
            )
            .map_err(|e| {
                EmbeddingError::Inference(format!("Failed to create input_ids tensor: {}", e))
            })?,
        );

        let attention = Tensor::from(
            Array2::<i64>::from_shape_vec(
                (1, seq_len),
                attention_mask.iter().map(|&id| id as i64).collect(),
            )
            .map_err(|e| {
                EmbeddingError::Inference(format!("Failed to create attention_mask tensor: {}", e))
            })?,
        );

        let token_type = Tensor::from(
            Array2::<i64>::from_shape_vec((1, seq_len), vec![0_i64; seq_len]).map_err(|e| {
                EmbeddingError::Inference(format!("Failed to create token_type_ids tensor: {}", e))
            })?,
        );

        let result = self
            .model
            .run(tvec!(input_ids.into(), attention.into(), token_type.into()))
            .map_err(|e| EmbeddingError::Inference(format!("Inference failed: {}", e)))?;

        let output = result
            .into_iter()
            .next()
            .ok_or_else(|| EmbeddingError::Inference("No output".to_string()))?;

        let slice = output
            .as_slice()
            .map_err(|e| EmbeddingError::Inference(format!("Failed to get tensor slice: {}", e)))?;

        let pooled = self.mean_pool(slice, seq_len);
        let normalized = self.l2_normalize(&pooled);

        Ok(normalized)
    }

    fn mean_pool(&self, embeddings: &[f32], seq_len: usize) -> Vec<f32> {
        let hidden_dim = self.dimension;

        let mut pooled = vec![0.0; hidden_dim];

        for j in 0..hidden_dim {
            let mut sum = 0.0f32;
            for i in 0..seq_len {
                sum += embeddings[i * hidden_dim + j];
            }
            pooled[j] = sum / seq_len as f32;
        }

        pooled
    }

    fn l2_normalize(&self, embedding: &[f32]) -> Vec<f32> {
        let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();

        if norm <= 0.0 {
            return embedding.to_vec();
        }

        embedding.iter().map(|&x| x / norm).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onnx_model_missing_file() {
        let result = OnnxModel::from_file("nonexistent_model.onnx");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, EmbeddingError::ModelLoad(_)));
    }

    #[test]
    #[ignore]
    fn test_full_pipeline() {
        let model_path = "../all-MiniLM-L6-v2/model.onnx";
        let tokenizer_path = "../all-MiniLM-L6-v2/tokenizer.json";

        let tokenizer = crate::model::HuggingFaceTokenizer::from_file(tokenizer_path)
            .expect("Failed to load tokenizer");

        let mut model = OnnxModel::from_file(model_path).expect("Failed to load model");

        let text = "Hello, world!";
        let encoding = tokenizer.encode(text).expect("Failed to encode");

        let embedding = model
            .infer(&encoding.ids, &encoding.attention_mask)
            .expect("Failed to infer");

        assert_eq!(embedding.len(), 384, "Embedding should have 384 dimensions");

        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 0.01,
            "Embedding should be L2 normalized"
        );

        for (i, val) in embedding.iter().enumerate() {
            assert!(val.is_finite(), "Embedding[{}] is not finite: {:?}", i, val);
        }
    }

    #[test]
    #[ignore]
    fn test_deterministic_output() {
        let model_path = "../all-MiniLM-L6-v2/model.onnx";
        let tokenizer_path = "../all-MiniLM-L6-v2/tokenizer.json";

        let tokenizer = crate::model::HuggingFaceTokenizer::from_file(tokenizer_path)
            .expect("Failed to load tokenizer");

        let mut model = OnnxModel::from_file(model_path).expect("Failed to load model");

        let text = "Test text for determinism";
        let encoding = tokenizer.encode(text).expect("Failed to encode");

        let embedding1 = model
            .infer(&encoding.ids, &encoding.attention_mask)
            .expect("Failed to infer");

        let embedding2 = model
            .infer(&encoding.ids, &encoding.attention_mask)
            .expect("Failed to infer");

        for (i, (v1, v2)) in embedding1.iter().zip(embedding2.iter()).enumerate() {
            assert!(
                (v1 - v2).abs() < 0.001,
                "Embedding[{}] differs: {} vs {}",
                i,
                v1,
                v2
            );
        }
    }
}
