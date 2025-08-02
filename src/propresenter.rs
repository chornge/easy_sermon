use serde_json::json;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[allow(dead_code)]
pub async fn stage_display(verse: &str) -> Result<(), Box<dyn Error>> {
    let host = "localhost";
    let port = 54346;
    let addr = format!("{}:{}", host, port);

    let request = json!({
        "url": "v1/stage/message",
        "method": "PUT",
        "body": verse,
        "chunked": false
    });

    let mut stream = TcpStream::connect(addr).await?;

    let mut request_str = request.to_string();
    request_str.push_str("\r\n"); // CRLF-terminated JSON

    stream.write_all(request_str.as_bytes()).await?;
    stream.shutdown().await?;

    println!("✅ {verse} is on Stage Display");
    
    Ok(())
}
