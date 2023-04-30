//! This module defines various types used throughout the ChatGPT API library.
//!
//! The types defined in this module include:
//! - `ChatInput`: The input parameters for generating responses from the ChatGPT model.
//! - `Chunk`, `ChunkChoice`, `Delta`: Types related to response streaming (when using the `stream` feature).
//! - `Client`: The main ChatGPT API client.
//! - `Message`, `Role`: Types related to messages and roles in conversations.
//! - `Model`: The available ChatGPT models.
//! - `Response`, `Choice`, `TokenUsage`: Types related to ChatGPT responses.
//! - `StreamItem`: Type for processing streamed responses (when using the `stream` feature).

pub mod chat_input;
#[cfg(feature = "stream")]
pub mod chunk;
pub mod client;
pub mod message;
pub mod model;
pub mod response;
#[cfg(feature = "stream")]
pub mod stream_item;
pub use chat_input::ChatInput;

#[cfg(feature = "stream")]
pub use chunk::{Chunk, ChunkChoice, Delta};
pub use client::Client;
pub use message::{Message, Role};
pub use model::Model;
pub use response::{Choice, Response, TokenUsage};
#[cfg(feature = "stream")]
pub use stream_item::StreamItem;
