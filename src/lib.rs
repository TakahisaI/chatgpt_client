pub mod err;
pub mod types;

pub use err::Error;
pub use types::{ChatInput, Client, Message, Model, Response, Role, TokenUsage};

pub type Result<T> = std::result::Result<T, Error>;
