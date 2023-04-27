use std::ops::{Deref, DerefMut};

use crate::types::{ChatInput, Error, Response, Result};

#[derive(Debug, Clone)]
pub struct Client(reqwest::Client);

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Client {
    const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

    pub fn new(api_key: String) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self(client))
    }

    pub async fn send<'a>(&self, input: &ChatInput<'a>) -> Result<reqwest::Response> {
        Ok(self.post(Self::API_URL).body(input).send().await?)
    }

    pub async fn completion<'a>(&self, input: &ChatInput<'a>) -> Result<Response> {
        let response = self.send(input).await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json::<Response>().await?)
        } else {
            let status_code = response.status();
            let headers = response.headers().to_owned();
            let body = response.text().await?;
            Err(Error::RequestFailed(status_code, headers, body))
        }
    }
}
