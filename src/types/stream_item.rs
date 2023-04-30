/// Represents an item in the stream of responses from the ChatGPT API when using the `stream` feature.
///
/// This enum is used when processing streaming responses from the ChatGPT API. It handles the start, content,
/// and finish reason for each chunk in the stream.
#[derive(Debug)]
pub enum StreamItem {
    /// Represents the start of a new response chunk.
    ///
    /// This variant contains metadata about the response chunk, such as the unique identifier, object type,
    /// creation timestamp, model, and role.
    Start {
        id: String,
        object: String,
        created: u64,
        model: String,
        role: crate::types::Role,
    },
    /// Represents the content of the response chunk.
    ///
    /// This variant contains the generated text content from the model.
    Content(String),
    /// Represents the finish reason for the response chunk.
    ///
    /// This variant contains the reason for finishing the generation, such as "stop" (reached stop sequence),
    /// "length" (reached max tokens), or "eos" (end of sentence).
    FinishReason(String),
}

/// Implements the conversion from a `Chunk` to a `StreamItem`.
///
/// This implementation is used to convert the data received from the ChatGPT API into a more manageable `StreamItem`
/// format that can be processed by the client.
impl From<crate::types::Chunk> for StreamItem {
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
