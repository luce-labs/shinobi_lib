use crate::types::ProtectedSecret;
use serde_json::json;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn get_env(key: &str) -> Result<ProtectedSecret, Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:6000").await?;

    let request = format!("(\"get_env\", \"{}\")", key);
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    if n == 0 {
        return Err("No response from server".into());
    }

    let response = String::from_utf8_lossy(&buffer[..n]).to_string();

    let secret = match response.trim() {
        "None" => ProtectedSecret::new(None),
        value if !value.is_empty() => ProtectedSecret::new(Some(value.to_string())),
        _ => ProtectedSecret::new(None),
    };

    Ok(secret)
}

pub async fn store_env(
    secrets: Vec<(String, Vec<String>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    if secrets.is_empty() {
        return Err("At least one secret must be provided".into());
    }

    let message = json!(["store_env", secrets]).to_string();

    let mut stream = TcpStream::connect("127.0.0.1:6000").await?;
    println!("Connected to server");

    stream.write_all(message.as_bytes()).await?;
    println!("Message sent");

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Server response: {}", response);

    Ok(())
}
