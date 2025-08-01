use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[allow(dead_code)]
pub async fn stage_display(verse: &str) -> Result<(), Box<dyn Error>> {
    let host = "localhost";
    let port = 49279;
    let client = Client::new();
    let response = client
        .put(format!("http://{host}:{port}/v1/stage/message"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json!(verse))
        .send()
        .await?;

    println!("Verse sent, Response: {response:?}");
    Ok(())
}
