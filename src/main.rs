use reqwest::Client;
use serde_json::json;
use std::{error::Error, process::Command};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

fn main() {
    let stream = Command::new("uvicorn")
        .arg("api.main:app")
        .arg("--host")
        .arg("0.0.0.0")
        .arg("--port")
        .arg("80")
        .arg("--reload")
        .status()
        .expect("failed to run api/stream.py");

    if !stream.success() {
        panic!("Run failed");
    }
}

async fn _send_to_server(verse: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:PORT/")
        .json(&json!({ "bible_verse": verse }))
        .send()
        .await?;

    println!("Verse sent, Response: {:?}", response);
    Ok(())
}

async fn _send_to_propresenter(verse: &str, host: &str, port: u16) -> Result<(), Box<dyn Error>> {
    // Create a TCP connection to ProPresenter
    let mut client_socket = TcpStream::connect((host, port)).await?;

    // Craft command for searching a Bible verse
    let command = json!({
        "action": "bible.verseSearch",
        "parameters": {
            "verse": verse
        }
    });

    // Send command
    let command_str = serde_json::to_string(&command)?;
    client_socket.write_all(command_str.as_bytes()).await?;

    // Receive response
    let mut buffer = [0; 4096];
    let n = client_socket.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Verse sent, Response: {:?}", response);
    Ok(())
}
