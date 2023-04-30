//! A library for interacting with the ChatGPT API.
//!
//! This library provides an easy-to-use interface for generating text completions using
//! the ChatGPT API. The main components are:
//! - `Client`: The main API client for interacting with the ChatGPT API.
//! - `ChatInput`: A struct for specifying the input parameters for a ChatGPT request.
//! - `Message`, `Role`: Types related to messages and roles in conversations.
//! - `Model`: Enum representing the available ChatGPT models.
//! - `Response`: A struct representing the ChatGPT API response.
//! - `TokenUsage`: A struct containing information about token usage in the response.
//!
//! # Example
//!
//! ```rust
//! use chatgpt::Client;
//! use chatgpt::types::{ChatInput, Message, Model, Role};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("OPENAI_API_KEY").unwrap();
//!     let client = Client::new(api_key);
//!
//!     let input = ChatInput {
//!         model: Model::Gpt4,
//!         messages: &[
//!             Message::user("What is the capital of France?"),
//!             Message::assistant("The capital of France is Paris."),
//!             Message::user("Tell me more about Paris."),
//!         ],
//!         ..Default::default()
//!     };
//!
//!     let response = client.completion(&input).await.unwrap();
//!     let assistant_message = &response.choices[0].message;
//!
//!     assert_eq!(assistant_message.role, Role::Assistant);
//!     println!("Response: {}", assistant_message.content);
//! }
//! ```

pub mod err;
pub mod types;

pub use err::Error;
pub use types::{ChatInput, Client, Message, Model, Response, Role, TokenUsage};

pub type Result<T> = std::result::Result<T, Error>;
