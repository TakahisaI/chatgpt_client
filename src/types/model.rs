use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub enum Model {
    #[default]
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-32k")]
    Gpt4_32k,
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let model_name = match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo",
            Model::Gpt4 => "gpt-4",
            Model::Gpt4_32k => "gpt-4-32k",
        };
        write!(f, "{model_name}")
    }
}
