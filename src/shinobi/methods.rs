use crate::types::ProtectedSecret;
use byteorder::{NetworkEndian, ReadBytesExt};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn get_env(keys: &[&str]) -> Result<HashMap<String, ProtectedSecret>, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6000")?;

    let mut request = vec!["get_env".to_string()];
    request.extend(keys.iter().map(|&k| k.to_string()));

    let request_json = serde_json::to_string(&request)?;

    stream.write_all(request_json.as_bytes())?;

    let response_len = stream.read_u32::<NetworkEndian>()?;

    if response_len == 0 {
        return Ok(HashMap::new());
    }

    let mut response_buffer = vec![0; response_len as usize];
    stream.read_exact(&mut response_buffer)?;

    let secrets_map: HashMap<String, String> = serde_json::from_slice(&response_buffer)?;

    let protected_secrets = secrets_map
        .into_iter()
        .map(|(k, v)| (k, ProtectedSecret::new(Some(v))))
        .collect();

    Ok(protected_secrets)
}

// pub async fn store_env(
//     secrets: Vec<(String, Vec<String>)>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     if secrets.is_empty() {
//         return Err("At least one secret must be provided".into());
//     }

//     let message = json!(["store_env", secrets]).to_string();

//     let mut stream = TcpStream::connect("127.0.0.1:6000").await?;
//     println!("Connected to server");

//     stream.write_all(message.as_bytes()).await?;
//     println!("Message sent");

//     let mut buffer = vec![0; 1024];
//     let n = stream.read(&mut buffer).await?;
//     let response = String::from_utf8_lossy(&buffer[..n]);
//     println!("Server response: {}", response);

//     Ok(())
// }
