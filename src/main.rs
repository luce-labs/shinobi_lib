pub mod shinobi;
pub mod types;

use crate::shinobi::shinobi::{get_env, store_env};

#[tokio::main]
async fn main() {
    let key = "DB_PASSWORD";

    let secrets = vec![
        (
            "DB_PASSWORD".to_string(),
            vec!["super_secret_123".to_string()],
        ),
        (
            "API_KEY".to_string(),
            vec!["very_secret_key_456".to_string()],
        ),
    ];

    match store_env(secrets).await {
        Ok(_) => println!("Secrets stored successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }

    match get_env(key).await {
        Ok(secret) => {
            if secret.get_value().unwrap() == "super_secret_123" {
                println!("Secret: {:?}", secret.get_value().unwrap());
            } else {
                println!("No secret found for key '{}'", key);
            }
        }
        Err(e) => {
            eprintln!("Error retrieving secret: {}", e);
        }
    }
}
