use crate::types::ProtectedSecret;
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
