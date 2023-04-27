use crate::{Message, Model};
use reqwest::Body;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChatInput<'a> {
    pub model: Model,
    pub messages: &'a [Message<&'a str>],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatInput<'_> {
    fn default() -> Self {
        Self {
            model: Model::default(),
            messages: &[],
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }
}

impl From<&ChatInput<'_>> for Body {
    fn from(chat_input: &ChatInput) -> Self {
        Body::from(serde_json::to_string(chat_input).unwrap())
    }
}
