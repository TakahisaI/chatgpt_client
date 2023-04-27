use crate::types::{ChatInput, Error, Response, Result};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "stream")]
use crate::types::StreamItem;
#[cfg(feature = "stream")]
use eventsource_stream::{Event, Eventsource};
#[cfg(feature = "stream")]
use futures::stream::{unfold, Stream, StreamExt};

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
        let response = self.post(Self::API_URL).body(input).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let status_code = response.status();
            let headers = response.headers().to_owned();
            let body = response.text().await?;
            Err(Error::RequestFailed(status_code, headers, body))
        }
    }

    pub async fn completion<'a>(&self, input: &ChatInput<'a>) -> Result<Response> {
        Ok(self.send(input).await?.json::<Response>().await?)
    }

    #[cfg(feature = "stream")]
    pub async fn stream<'a>(
        &self,
        input: &ChatInput<'a>,
    ) -> Result<impl Stream<Item = Result<StreamItem<String>>>> {
        let stream = self.send(input).await?.bytes_stream().eventsource();
        Ok(unfold(stream, move |mut stream| async move {
            while let Some(Ok(Event { data, .. })) = stream.next().await {
                if data == "[DONE]" {
                    continue;
                }
                match serde_json::from_str::<crate::types::Chunk>(&data) {
                    Ok(chunk) => return Some((Ok(chunk.into()), stream)),
                    Err(err) => return Some((Err(err.into()), stream)),
                }
            }
            None
        }))
    }
}
