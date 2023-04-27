pub mod chat_input;
pub mod client;
pub mod err;
pub mod message;
pub mod model;
pub mod response;

pub use chat_input::ChatInput;
pub use client::Client;
pub use err::Error;
pub use message::{Message, Role};
pub use model::Model;
pub use response::{Choice, Response, TokenUsage};
pub type Result<T> = std::result::Result<T, Error>;
