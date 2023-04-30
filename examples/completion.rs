use chatgpt_client::{ChatInput, Client, Message, Result, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new(api_key)?;
    let input = ChatInput {
        messages: &[Message::system("Hello, World!")],
        ..Default::default()
    };
    let response = client.completion(&input).await?;
    let assistant_message = &response.choices[0].message;

    assert_eq!(assistant_message.role, Role::Assistant);
    println!("Response: {}", assistant_message.content);
    Ok(())
}
