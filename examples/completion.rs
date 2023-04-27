use chatgpt_client::{ChatInput, Client, Message, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new(api_key)?;
    let messages = vec![Message::system("Hello, World!")];
    let input = ChatInput {
        messages: &messages,
        ..Default::default()
    };

    let response = client.completion(&input).await?;
    println!("{:#?}", response);
    Ok(())
}
