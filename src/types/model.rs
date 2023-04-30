use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

/// Represents the available language models for generating responses.
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub enum Model {
    /// The GPT-3.5-turbo model.
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    /// The default GPT-4 model.
    #[default]
    #[serde(rename = "gpt-4")]
    Gpt4,
    /// The GPT-4 model with 32k tokens.
    #[serde(rename = "gpt-4-32k")]
    Gpt4_32k,
}

impl Display for Model {
    /// Formats the `Model` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The mutable formatter.
    ///
    /// # Returns
    ///
    /// A `Result` containing the formatted model name or an error.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let model_name = match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo",
            Model::Gpt4 => "gpt-4",
            Model::Gpt4_32k => "gpt-4-32k",
        };
        write!(f, "{model_name}")
    }
}
