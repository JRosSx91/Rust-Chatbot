#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::env;
use std::io;

#[derive(Serialize, Deserialize)]
struct Prompt {
    prompt: String,
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Get the API key from an environment variable
    let api_key: String = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");
    let stdin: io::Stdin = io::stdin();
    let mut message: String = String::new();
    println!("Ask something to Chat GPT");
    stdin
        .read_line(&mut message)
        .expect("Cannot read this line!");
    let prompt: Prompt = Prompt {
        prompt: format!("'{}'", message),
        max_tokens: 60,
    };

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res: reqwest::Response = client
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
