use crate::shinobi::key_exchange::DHKeyExchange;
use crate::types::ProtectedSecret;
use byteorder::{NetworkEndian, ReadBytesExt};
use num_bigint::BigUint;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn get_env(keys: &[&str]) -> Result<HashMap<String, ProtectedSecret>, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6000")?;

    let dh_exchange = DHKeyExchange::new();
    let client_public_key = dh_exchange.get_public_key().to_bytes_be();

    // Send client's public key
    stream.write_all(&(client_public_key.len() as u32).to_be_bytes())?;
    stream.write_all(&client_public_key)?;

    // Receive server's public key
    let mut server_key_length = [0u8; 4];
    stream.read_exact(&mut server_key_length)?;
    let server_key_length = u32::from_be_bytes(server_key_length) as usize;

    let mut server_public_key_bytes = vec![0u8; server_key_length];
    stream.read_exact(&mut server_public_key_bytes)?;

    let server_public_key = BigUint::from_bytes_be(&server_public_key_bytes);

    // Compute shared secret
    let shared_secret = dh_exchange.compute_shared_secret(&server_public_key);

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

    // Decrypt the response
    let decrypted_response = DHKeyExchange::decrypt(&shared_secret, &response_buffer);
    let response_json = String::from_utf8(decrypted_response)?;

    let secrets_map: HashMap<String, ProtectedSecret> =
        serde_json::from_str(&response_json).unwrap();

    let protected_secrets = secrets_map
        .into_iter()
        .map(|(k, v)| (k, ProtectedSecret::new(Some(v.get_value().unwrap()))))
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
