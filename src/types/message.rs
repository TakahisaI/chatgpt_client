use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Message<T> {
    pub role: Role,
    pub content: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<T>,
}

impl<T> Message<T> {
    pub fn system(content: T) -> Self {
        Self {
            role: Role::System,
            content: content,
            name: None,
        }
    }

    pub fn user(content: T) -> Self {
        Self {
            role: Role::User,
            content,
            name: None,
        }
    }

    pub fn assistant(content: T) -> Self {
        Self {
            role: Role::Assistant,
            content,
            name: None,
        }
    }
}
