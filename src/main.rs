mod devices;

use reqwest::Client;
use serde_json::json;
use std::{error::Error, process::Command};

fn main() {
    use devices::list_input_outputs;
    list_input_outputs();

    let stream = Command::new("uvicorn")
        .arg("api.main:app")
        .arg("--host")
        .arg("0.0.0.0")
        .arg("--port")
        .arg("80")
        .arg("--log-level")
        .arg("warning")
        .status()
        .expect("Failed to run api/main.py");

    if !stream.success() {
        panic!("Run failed");
    }
}

#[allow(dead_code)]
async fn stage_display(verse: &str) -> Result<(), Box<dyn Error>> {
    let pro7_p_host = "localhost";
    let pro7_p_port = 49279;
    let client = Client::new();
    let response = client
        .put(format!(
            "http://{pro7_p_host}:{pro7_p_port}/v1/stage/message"
        ))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json!(verse))
        .send()
        .await?;

    println!("Verse sent, Response: {response:?}");
    Ok(())
}
