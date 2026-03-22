use crate::config::ChunkerConfig;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub content: String,
    pub start_char: usize,
    pub end_char: usize,
    pub chunk_index: usize,
}

pub struct TextChunker {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl TextChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }

    pub fn from_config(config: &ChunkerConfig) -> Self {
        Self::new(config.chunk_size, config.chunk_overlap)
    }

    pub fn chunk(&self, text: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let text_len = text.chars().count();

        if text_len == 0 {
            return chunks;
        }

        let mut start = 0;
        let mut chunk_index = 0;

        while start < text_len {
            let end = (start + self.chunk_size).min(text_len);

            let (chunk_content, _, _) = text.chars().skip(start).take(end - start).fold(
                (String::new(), false, 0),
                |(mut s, in_word, count), c| {
                    if start + count >= text_len {
                        return (s, in_word, count);
                    }
                    s.push(c);
                    let new_in_word = c.is_alphanumeric() || c == '_';
                    (s, new_in_word, count + 1)
                },
            );

            if !chunk_content.trim().is_empty() {
                chunks.push(Chunk {
                    content: chunk_content.trim().to_string(),
                    start_char: start,
                    end_char: end,
                    chunk_index,
                });
            }

            chunk_index += 1;

            if end >= text_len {
                break;
            }

            start = end - self.chunk_overlap.min(end);
            if start <= chunks.last().map(|c| c.end_char).unwrap_or(0) {
                start = chunks.last().map(|c| c.end_char).unwrap_or(0);
                if start >= text_len {
                    break;
                }
            }
        }

        chunks
    }

    pub fn chunk_with_source(&self, text: &str, source: &str) -> Vec<(Chunk, String)> {
        self.chunk(text)
            .into_iter()
            .map(|chunk| (chunk, source.to_string()))
            .collect()
    }
}

impl Default for TextChunker {
    fn default() -> Self {
        Self::new(1000, 200)
    }
}

impl Clone for TextChunker {
    fn clone(&self) -> Self {
        Self {
            chunk_size: self.chunk_size,
            chunk_overlap: self.chunk_overlap,
        }
    }
}

pub struct SemanticChunker {
    base_chunker: TextChunker,
}

impl SemanticChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            base_chunker: TextChunker::new(chunk_size, chunk_overlap),
        }
    }

    pub fn chunk_by_paragraphs(&self, text: &str) -> Vec<Chunk> {
        let paragraphs: Vec<&str> = text
            .split(|c: char| c == '\n' && c.is_whitespace())
            .filter(|p: &&str| !p.trim().is_empty())
            .collect();

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_len = 0;
        let mut chunk_index = 0;
        let mut global_start = 0;

        for paragraph in paragraphs {
            let para_len = paragraph.chars().count();

            if current_len + para_len + 1 > self.base_chunker.chunk_size
                && !current_chunk.is_empty()
            {
                chunks.push(Chunk {
                    content: current_chunk.trim().to_string(),
                    start_char: global_start,
                    end_char: global_start + current_chunk.chars().count(),
                    chunk_index,
                });
                chunk_index += 1;

                global_start += current_chunk.chars().count();
                current_chunk.clear();
                current_len = 0;
            }

            if !current_chunk.is_empty() {
                current_chunk.push('\n');
                current_len += 1;
            }
            current_chunk.push_str(paragraph);
            current_len += para_len;
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(Chunk {
                content: current_chunk.trim().to_string(),
                start_char: global_start,
                end_char: global_start + current_chunk.chars().count(),
                chunk_index,
            });
        }

        chunks
    }

    pub fn chunk(&self, text: &str) -> Vec<Chunk> {
        self.chunk_by_paragraphs(text)
    }
}

impl Default for SemanticChunker {
    fn default() -> Self {
        Self::new(1000, 200)
    }
}

impl Clone for SemanticChunker {
    fn clone(&self) -> Self {
        self.base_chunker.clone().into()
    }
}

impl From<TextChunker> for SemanticChunker {
    fn from(base_chunker: TextChunker) -> Self {
        Self { base_chunker }
    }
}
