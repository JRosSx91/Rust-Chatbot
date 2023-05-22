#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::error::Error;

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

    // Create a client
    let client = reqwest::Client::new();

    // Define the prompt and max tokens
    let prompt = "Translate the following English text to French: '{}'";
    let max_tokens = 60;

    // Build the JSON body
    let body = json!({
        "prompt": prompt,
        "max_tokens": max_tokens
    });

    // Send the request
    let res = client
        .post("https://api.openai.com/v1/engines/text-davinci-002/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    // Print the response status and text
    println!("Status: {}", res.status());
    let text: String = res.text().await?;
    println!("Response:\n{}", text);

    Ok(())
}
