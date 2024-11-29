use crate::types::ProtectedSecret;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Retrieve an environment secret from the server.
/// This function connects to the server, sends the request, and processes the response.
pub async fn get_env(key: &str) -> Result<ProtectedSecret, Box<dyn Error>> {
    // Connect to the secrets server asynchronously using tokio
    let mut stream = TcpStream::connect("localhost:6000").await?;

    // Send the request to the server
    let request = format!("(\"get_env\", \"{}\")", key);
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?; // Make sure all data is sent

    // Read the server's response
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    // Handle case where no data was received
    if n == 0 {
        return Err("No response from server".into());
    }

    // Convert response to a string
    let response = String::from_utf8_lossy(&buffer[..n]).to_string();

    // Handle different possible responses
    let secret = match response.trim() {
        "None" => ProtectedSecret::new(None), // No secret found
        value if !value.is_empty() => ProtectedSecret::new(Some(value.to_string())), // Found secret
        _ => ProtectedSecret::new(None),      // Invalid response
    };

    Ok(secret)
}
