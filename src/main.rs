mod devices;
mod stream;

use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::{error::Error, process::Command};

// use crate::stream::start as start_streaming;
use dotenv::dotenv;
use std::env;

fn main() -> Result<()> {
    // Load .env if present
    dotenv().ok();
    env::var("VOSK_MODEL_PATH")
        .expect("Set VOSK_MODEL_PATH environment variable to the Vosk model directory path");

    // Start audio stream
    // let audio_stream = start_streaming();
    // if audio_stream.is_err() {
    //     panic!(
    //         "Failed to start audio stream: {}",
    //         audio_stream.unwrap_err()
    //     );
    // }

    // Start audio API server
    let audio_server = Command::new("uvicorn")
        .arg("api.main:app")
        .arg("--host")
        .arg("0.0.0.0")
        .arg("--port")
        .arg("80")
        .arg("--log-level")
        .arg("warning")
        .status()
        .expect("Failed to run api/main.py");
    if !audio_server.success() {
        panic!("API server failed to start");
    }

    Ok(())
}

async fn _stage_display(verse: &str) -> Result<(), Box<dyn Error>> {
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
