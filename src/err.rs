use reqwest::{header::HeaderMap, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invaild Header Value: {0}")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Reqwest client error: {0}")]
    ReqwestClientError(#[from] reqwest::Error),

    #[error("Reqwest Failed: \n\t status code: {0}\n\t headers: {1:?}\n\t body: {2}")]
    RequestFailed(StatusCode, HeaderMap, String),

    #[error("Response error: {0}")]
    ResponseError(String),

    #[cfg(feature = "stream")]
    #[error("Event stream error: {0}")]
    EventStreamError(#[from] eventsource_stream::EventStreamError<reqwest::Error>),

    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
