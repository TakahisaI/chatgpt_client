use crate::types::{ChatInput, Error, Response, Result};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "stream")]
use crate::types::{Chunk, ChunkChoice, Delta, StreamItem};
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

    pub async fn send(&self, input: &ChatInput<'_>) -> Result<reqwest::Response> {
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
    pub async fn stream(
        &self,
        input: &ChatInput<'_>,
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

    #[cfg(feature = "stream")]
    pub async fn compress(
        &self,
        input: &ChatInput<'_>,
        delimiter: &str,
    ) -> Result<impl Stream<Item = Result<StreamItem<String>>>> {
        let stream = self.send(input).await?.bytes_stream().eventsource();

        Ok(unfold(
            (stream, String::new(), delimiter.to_string()),
            move |(mut stream, mut buffer, delimiter)| async move {
                async fn process_event(
                    result: std::result::Result<
                        Event,
                        eventsource_stream::EventStreamError<reqwest::Error>,
                    >,
                    buffer: &mut String,
                    delimiter: &str,
                ) -> Result<Option<StreamItem<String>>> {
                    let Event { data, .. } = result.map_err(Error::from)?;

                    if data == "[DONE]" {
                        return Ok(Some(StreamItem::FinishReason(buffer.to_owned())));
                    }

                    let Chunk {
                        id,
                        object,
                        created,
                        model,
                        mut choices,
                    } = serde_json::from_str(&data).map_err(Error::from)?;

                    match choices.pop() {
                        Some(ChunkChoice {
                            delta: Delta { content, role },
                            finish_reason,
                            ..
                        }) => {
                            if let Some(mut reason) = finish_reason {
                                std::mem::swap(&mut reason, buffer);
                                Ok(Some(StreamItem::Content(reason)))
                            } else if let Some(content) = content {
                                buffer.push_str(&content);
                                if let Some(index) = buffer.find(delimiter) {
                                    let content_before_delimiter =
                                        buffer.drain(..index).collect::<String>();
                                    buffer.drain(..delimiter.len());

                                    Ok(Some(StreamItem::Content(content_before_delimiter)))
                                } else {
                                    Ok(None)
                                }
                            } else if let Some(role) = role {
                                Ok(Some(StreamItem::Start {
                                    id,
                                    object,
                                    created,
                                    model,
                                    role,
                                }))
                            } else {
                                Err(Error::ResponseError(
                                    "There is no content or role in response".to_string(),
                                ))
                            }
                        }
                        None => Err(Error::ResponseError(
                            "There is no choice in response".to_string(),
                        )),
                    }
                }

                while let Some(result) = stream.next().await {
                    match process_event(result, &mut buffer, &delimiter).await {
                        Ok(Some(stream_item)) => {
                            return Some((Ok(stream_item), (stream, buffer, delimiter)));
                        }
                        Ok(None) => continue,
                        Err(err) => return Some((Err(err), (stream, buffer, delimiter))),
                    }
                }
                None
            },
        ))
    }
}
