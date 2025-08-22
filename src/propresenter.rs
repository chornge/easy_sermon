use serde_json::json;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

#[allow(dead_code)]
pub async fn stage_display(verse: &str) -> Result<(), Box<dyn Error>> {
    let host = "localhost";
    let port = 54346;
    let addr = format!("{host}:{port}");

    let request = json!({
        "url": "v1/stage/message",
        "method": "PUT",
        "body": verse,
        "chunked": false
    });

    // Convert to string and append CRLF
    let mut request_str = request.to_string();
    request_str.push_str("\r\n");

    // Connect with a timeout of 3 seconds
    let stream = match timeout(Duration::from_secs(3), TcpStream::connect(addr)).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => {
            eprintln!("❌ Connection error: {}", e);
            return Err(Box::new(e));
        }
        Err(_) => {
            eprintln!("❌ Connection timed out");
            return Err("Connection timed out".into());
        }
    };

    // Split the TcpStream into writer half (to be explicit about usage)
    let (_, mut writer) = tokio::io::split(stream);

    if let Err(e) = writer.write_all(request_str.as_bytes()).await {
        eprintln!("❌ Write error: {}", e);
        return Err(Box::new(e));
    }

    // Flush and shutdown the writer explicitly
    if let Err(e) = writer.shutdown().await {
        eprintln!("❌ Shutdown error: {}", e);
        return Err(Box::new(e));
    }

    println!("✅ {verse} is on Stage Display");

    Ok(())
}
