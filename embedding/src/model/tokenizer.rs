use crate::error::{EmbeddingError, Result};
use std::sync::Arc;
use tokenizers::Tokenizer;

pub struct HuggingFaceTokenizer {
    tokenizer: Arc<Tokenizer>,
    max_length: usize,
}

impl HuggingFaceTokenizer {
    pub fn from_file(path: &str) -> Result<Self> {
        let tokenizer =
            Tokenizer::from_file(path).map_err(|e| EmbeddingError::TokenizerLoad(e.to_string()))?;

        let max_length = 512;

        Ok(Self {
            tokenizer: Arc::new(tokenizer),
            max_length,
        })
    }

    pub fn encode(&self, text: &str) -> Result<Encoding> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| EmbeddingError::Tokenization(e.to_string()))?;

        Ok(Encoding::from(encoding))
    }

    pub fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Encoding>> {
        let encodings = self
            .tokenizer
            .encode_batch(
                texts.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                true,
            )
            .map_err(|e| EmbeddingError::Tokenization(e.to_string()))?;

        Ok(encodings.into_iter().map(Encoding::from).collect())
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }
}

#[derive(Debug, Clone)]
pub struct Encoding {
    pub ids: Vec<u32>,
    pub tokens: Vec<String>,
    pub offsets: Vec<(usize, usize)>,
    pub attention_mask: Vec<u32>,
}

impl Encoding {
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn truncate(&mut self, max_length: usize) {
        if self.ids.len() > max_length {
            self.ids.truncate(max_length);
            self.tokens.truncate(max_length);
            self.offsets.truncate(max_length);
            self.attention_mask.truncate(max_length);
        }
    }
}

impl From<tokenizers::Encoding> for Encoding {
    fn from(encoding: tokenizers::Encoding) -> Self {
        Self {
            ids: encoding.get_ids().to_vec(),
            tokens: encoding.get_tokens().to_vec(),
            offsets: encoding.get_offsets().to_vec(),
            attention_mask: encoding.get_attention_mask().to_vec(),
        }
    }
}
