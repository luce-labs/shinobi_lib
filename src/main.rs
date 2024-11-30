pub mod shinobi;
pub mod types;

use crate::shinobi::methods::get_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let key = "API_KEY";

    // let secrets = vec![
    //     (
    //         "DB_PASSWORD".to_string(),
    //         vec!["super_secret_123".to_string()],
    //     ),
    //     (
    //         "API_KEY".to_string(),
    //         vec!["very_secret_key_456".to_string()],
    //     ),
    // ];

    // match store_env(secrets).await {
    //     Ok(_) => println!("Secrets stored successfully"),
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    let env_vars = get_env(&["DB_PASSWORD", "API_KEY"])?;

    for (key, secret) in env_vars.iter() {
        if let Some(value) = secret.get_value() {
            println!("{}: {}", key, value);
            if value == "very_secret_key_456" {
                println!("API_KEY found!");
            }
        } else {
            println!("{}: No value found", key);
        }
    }

    Ok(())
}
