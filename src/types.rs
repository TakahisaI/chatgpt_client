pub mod chat_input;
#[cfg(feature = "stream")]
pub mod chunk;
pub mod client;
pub mod err;
pub mod message;
pub mod model;
pub mod response;
#[cfg(feature = "stream")]
pub mod stream_item;
pub use chat_input::ChatInput;

#[cfg(feature = "stream")]
pub use chunk::{Chunk, ChunkChoice, Delta};
pub use client::Client;
pub use err::Error;
pub use message::{Message, Role};
pub use model::Model;
pub use response::{Choice, Response, TokenUsage};
#[cfg(feature = "stream")]
pub use stream_item::StreamItem;
pub type Result<T> = std::result::Result<T, Error>;
