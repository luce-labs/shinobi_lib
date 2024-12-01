pub mod shinobi;
pub mod types;

use crate::shinobi::methods::{get_env, store_env};
use std::collections::HashMap;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut secrets = HashMap::new();
    secrets.insert("DB_USERNAME".to_string(), "my_database_user".to_string());
    secrets.insert(
        "DB_PASSWORD".to_string(),
        "my_super_secret_password".to_string(),
    );
    secrets.insert("API_KEY".to_string(), "my_api_key_value".to_string());

    store_env(secrets)?;
    println!("Secrets stored successfully");

    let retrieved_secrets = get_env(&["DB_USERNAME", "API_KEY"])?;

    for (key, secret) in retrieved_secrets.iter() {
        if let Some(value) = Some(secret.get_value().unwrap()) {
            if value == "my_database_user" {
                println!("{:?}", value)
            } else if value == "my_api_key_value" {
                println!("api key here")
            }
        }
    }

    Ok(())
}
