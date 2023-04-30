use crate::types::Role;
use serde::Deserialize;

/// Represents a change in the content and role of a response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct Delta {
    /// The role of the message sender, if changed.
    pub role: Option<Role>,
    /// The content of the message, if changed.
    pub content: Option<String>,
}

/// Represents a choice in the chunk of a response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct ChunkChoice {
    /// The index of the choice.
    pub index: usize,
    /// The change in content and role for this choice.
    pub delta: Delta,
    /// The reason for the response to finish, if applicable.
    pub finish_reason: Option<String>,
}

/// Represents a chunk in the response stream.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
pub struct Chunk {
    /// The unique identifier of the chunk.
    pub id: String,
    /// The type of the object.
    pub object: String,
    /// The timestamp of the creation of the chunk.
    pub created: u64,
    /// The model used to generate the chunk.
    pub model: String,
    /// A vector of choices in the chunk.
    pub choices: Vec<ChunkChoice>,
}
