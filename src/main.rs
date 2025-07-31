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
        .expect("failed to run api/stream.py");

    if !stream.success() {
        panic!("Run failed");
    }
}

#[allow(dead_code)]
async fn send_to_propresenter(verse: &str) -> Result<(), Box<dyn Error>> {
    let pro7_host = "localhost";
    let pro7_port = 49279;
    let client = Client::new();
    let response = client
        .put(format!("http://{pro7_host}:{pro7_port}/v1/stage/message"))
        .json(&json!(verse))
        .send()
        .await?;

    println!("Verse sent, Response: {response:?}");
    Ok(())
}
