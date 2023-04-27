use super::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: TokenUsage,
}

#[derive(Debug, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message<String>,
    pub finish_reason: String,
}
