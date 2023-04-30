use crate::{Message, Model};
use reqwest::Body;
use serde::Serialize;

/// Represents the input for a chat API call.
#[derive(Debug, Serialize)]
pub struct ChatInput<'a> {
    /// The model to use for generating responses.
    pub model: Model,
    /// The array of messages to send as input.
    pub messages: &'a [Message<&'a str>],
    /// The optional temperature to use for controlling randomness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// The optional top_p value for controlling sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// The optional number of generated choices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    /// The optional streaming flag.
    #[cfg(feature = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// The optional array of stop phrases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// The optional maximum number of tokens in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    /// The optional presence penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// The optional frequency penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// The optional logit bias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<i16>,
    /// The optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatInput<'_> {
    /// Provides default values for `ChatInput`.
    ///
    /// # Returns
    ///
    /// A `ChatInput` instance with default values.
    fn default() -> Self {
        Self {
            model: Model::default(),
            messages: &[],
            temperature: None,
            top_p: None,
            n: None,
            #[cfg(feature = "stream")]
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
    /// Converts a `ChatInput` into a `Body` for use in an API request.
    ///
    /// # Arguments
    ///
    /// * `chat_input` - The `ChatInput` to be converted.
    ///
    /// # Returns
    ///
    /// A `Body` containing the serialized JSON representation of the `ChatInput`.
    fn from(chat_input: &ChatInput) -> Self {
        Body::from(serde_json::to_string(chat_input).unwrap())
    }
}
