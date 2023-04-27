use chatgpt_client::{ChatInput, Client, Message, Result};
use futures::stream::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new(api_key)?;
    let messages = vec![Message::system("Hello, World!")];
    let input = ChatInput {
        messages: &messages,
        stream: Some(true),
        ..Default::default()
    };

    let stream = client.stream(&input).await?;
    stream
        .for_each(|result| {
            match result {
                Ok(item) => println!("{:?}", item),
                Err(error) => println!("{:?}", error),
            }
            async {}
        })
        .await;

    Ok(())
}