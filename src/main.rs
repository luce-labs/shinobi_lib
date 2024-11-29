pub mod shinobi;
pub mod types;

use crate::shinobi::shinobi::get_env;

#[tokio::main]
async fn main() {
    // Example usage
    let key = "DB_PASSWORD";

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
