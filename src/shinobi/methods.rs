use crate::types::ProtectedSecret;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn get_env(keys: &[&str]) -> Result<HashMap<String, ProtectedSecret>, Box<dyn Error>> {
    use byteorder::{NetworkEndian, ReadBytesExt};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:6000")?;

    let mut request = vec!["get_env".to_string()];
    request.extend(keys.iter().map(|&k| k.to_string()));
    let request_json = serde_json::to_string(&request)?;

    stream.write_all(request_json.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut length_buffer = [0u8; 4];
    stream.read_exact(&mut length_buffer)?;
    let response_length = (&length_buffer[..]).read_u32::<NetworkEndian>()? as usize;

    let mut response_buffer = vec![0; response_length];
    stream.read_exact(&mut response_buffer)?;

    let response_json = String::from_utf8(response_buffer)?;

    let secrets_map: HashMap<String, String> = serde_json::from_str(&response_json)?;

    let protected_secrets = secrets_map
        .into_iter()
        .map(|(k, v)| (k, ProtectedSecret::new(Some(v))))
        .collect();

    Ok(protected_secrets)
}

pub fn store_env(secrets: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6000")?;

    let mut request = vec!["store_env".to_string()];

    let secrets_json = serde_json::to_string(&secrets)?;
    request.push(secrets_json);

    let request_json = serde_json::to_string(&request)?;

    stream.write_all(request_json.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut ack_buffer = [0u8; 4];
    stream.read_exact(&mut ack_buffer)?;

    if ack_buffer != [0, 0, 0, 0] {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to store secrets",
        )));
    }

    Ok(())
}
