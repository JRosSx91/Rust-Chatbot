use reqwest;
use serde_json::json;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
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
