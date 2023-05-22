#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::env;

#[derive(Serialize, Deserialize)]
struct Prompt {
    prompt: String,
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Get the API key from an environment variable
    let api_key = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");
    let message = "Hello, world!";
    let prompt = Prompt {
        prompt: format!(
            "Translate the following English text to French: '{}'",
            message
        ),
        max_tokens: 60,
    };

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client
        .post("https://api.openai.com/v1/engines/text-davinci-002/completions")
        .headers(headers)
        .json(&prompt)
        .send()
        .await?;

    let response_text: serde_json::Value = res.json().await?;
    if let Some(text) = response_text["choices"][0]["text"].as_str() {
        println!("{}", text);
    }

    Ok(())
}
