pub mod shinobi;
pub mod types;

use crate::shinobi::methods::{get_env, store_env};
use std::collections::HashMap;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    // Prepare secrets to store
    let mut secrets = HashMap::new();
    secrets.insert("DB_USERNAME".to_string(), "my_database_user".to_string());
    secrets.insert(
        "DB_PASSWORD".to_string(),
        "my_super_secret_password".to_string(),
    );
    secrets.insert("API_KEY".to_string(), "my_api_key_value".to_string());

    // Store the secrets
    store_env(secrets)?;
    println!("Secrets stored successfully");

    // Retrieve specific secrets
    let retrieved_secrets = get_env(&["DB_USERNAME", "API_KEY"])?;

    // Print retrieved secrets
    for (key, secret) in retrieved_secrets.iter() {
        if let Some(value) = secret.get_value() {
            println!("{}: {}", key, value);
        }
    }

    Ok(())
}
