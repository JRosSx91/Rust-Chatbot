#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::env;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatCompletion {
    model: String,
    messages: Vec<Message>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key: String = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");
    println!("Enter your message: ");
    io::stdout().flush()?;
    let mut user_message: String = String::new();
    io::stdin().read_line(&mut user_message)?;
    user_message = user_message.trim().to_string();
    let chat: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message,
        }],
    };

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res: reqwest::Response = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&chat)
        .send()
        .await?;

    let response_text: serde_json::Value = res.json().await?;
    if let Some(text) = response_text["choices"][0]["message"]["content"].as_str() {
        println!("{}", text);
    }
    println!("\nPress Enter to exit...");

    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).expect("Failed to read");

    Ok(())
}
