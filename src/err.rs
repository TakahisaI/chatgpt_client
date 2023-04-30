use reqwest::{header::HeaderMap, StatusCode};
use thiserror::Error;

/// Defines the possible errors that can occur when interacting with the ChatGPT API.
///
/// This enum includes various error types that can occur during the request and response process
/// when interacting with the ChatGPT API. These errors include invalid header values, client errors,
/// failed requests, response errors, event stream errors (when using the `stream` feature), and JSON
/// serialization/deserialization errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents an invalid header value error.
    #[error("Invalid header value: {0}")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    /// Represents a reqwest client error.
    #[error("Reqwest client error occurred: {0}")]
    ReqwestClientError(#[from] reqwest::Error),

    /// Represents a failed request error.
    #[error("Request failed with status code: {0}, headers: {1:?}, body: {2}")]
    RequestFailed(StatusCode, HeaderMap, String),

    /// Represents an error that occurred while processing the response.
    #[error("Error occurred while processing the response: {0}")]
    ResponseError(String),

    /// Represents an event stream error (when using the `stream` feature).
    #[cfg(feature = "stream")]
    #[error("Event stream error occurred: {0}")]
    EventStreamError(#[from] eventsource_stream::EventStreamError<reqwest::Error>),

    /// Represents an error that occurred while processing JSON data.
    #[error("Error occurred while processing JSON data: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
