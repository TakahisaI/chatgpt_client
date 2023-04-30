use serde::{Deserialize, Serialize};

/// Represents the role of a message sender in the conversation.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// The system role, usually used for instructions or context.
    #[default]
    System,
    /// The user role, representing the end-user interacting with the API.
    User,
    /// The assistant role, representing the AI-generated responses.
    Assistant,
}

/// Represents a message with a specific role and content.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Message<T> {
    /// The role of the message sender.
    pub role: Role,
    /// The content of the message.
    pub content: T,
    /// The optional name of the sender, only used if needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<T>,
}

impl<T> Message<T> {
    /// Creates a new `Message` with the system role and the provided content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content for the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance with the system role.
    pub fn system(content: T) -> Self {
        Self {
            role: Role::System,
            content: content,
            name: None,
        }
    }

    /// Creates a new `Message` with the user role and the provided content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content for the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance with the user role.
    pub fn user(content: T) -> Self {
        Self {
            role: Role::User,
            content,
            name: None,
        }
    }

    /// Creates a new `Message` with the assistant role and the provided content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content for the message.
    ///
    /// # Returns
    ///
    /// A new `Message` instance with the assistant role.
    pub fn assistant(content: T) -> Self {
        Self {
            role: Role::Assistant,
            content,
            name: None,
        }
    }
}
