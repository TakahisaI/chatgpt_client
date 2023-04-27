#[derive(Debug)]
pub enum StreamItem<T> {
    Start {
        id: String,
        object: String,
        created: u64,
        model: String,
        role: crate::types::Role,
    },
    Content(T),
    FinishReason(String),
}

impl From<crate::types::Chunk> for StreamItem<String> {
    fn from(mut chunk: crate::types::Chunk) -> Self {
        let choice = chunk.choices.first_mut().unwrap();
        if let Some(role) = choice.delta.role.take() {
            StreamItem::Start {
                id: chunk.id,
                object: chunk.object,
                created: chunk.created,
                model: chunk.model,
                role,
            }
        } else if let Some(content) = choice.delta.content.take() {
            StreamItem::Content(content)
        } else if let Some(reason) = choice.finish_reason.take() {
            StreamItem::FinishReason(reason)
        } else {
            panic!("Unexpected response chunk: {:?}", chunk);
        }
    }
}

impl StreamItem<String> {
    pub fn parse_json<T>(self) -> crate::Result<StreamItem<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            StreamItem::Start {
                id,
                object,
                created,
                model,
                role,
            } => Ok(StreamItem::Start {
                id,
                object,
                created,
                model,
                role,
            }),
            StreamItem::Content(content) => {
                Ok(StreamItem::Content(serde_json::from_str(&content)?))
            }
            StreamItem::FinishReason(reason) => Ok(StreamItem::FinishReason(reason)),
        }
    }
}