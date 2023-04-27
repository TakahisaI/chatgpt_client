use crate::types::Role;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct Delta {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct ChunkChoice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}
