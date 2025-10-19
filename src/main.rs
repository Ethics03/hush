use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod models;

mod vault;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => {
            // Create vault directory
            if let Err(e) = vault::create_vault_dir(&name) {
                eprintln!("Error: {}", e);
                return;
            }

            // getting timestamp for metadata
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            // creating vault metadata
            let vault = models::Vault {
                name: name.clone(),
                created_at: timestamp,
                updated_at: timestamp,
            };

            // saving vault
            if let Err(e) = vault::save_vault(&vault) {
                eprintln!("Error: {}", e);
                return;
            }

            // creating empty secrets.json
            let empty_secrets = std::collections::HashMap::new();
            if let Err(e) = vault::save_secret(&name, &empty_secrets) {
                eprintln!("Error: {}", e);
                return;
            }

            println!("Vault '{}' created successfully", name);
        }

        Commands::Set { vault, key, value } => {
            let mut secrets = match vault::load_secret(&vault) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let secret = models::Secret {
                key: key.clone(),
                value: value.clone(),
                created_at: timestamp,
            };

            //insert into hashmap
            secrets.insert(key.clone(), secret);

            match vault::save_secret(&vault, &secrets) {
                Ok(_) => println!("Secret: '{}' saved to vault '{}", key, vault),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        //loadng the secrets from the vault
        Commands::Get { vault, key } => {
            let secrets = match vault::load_secret(&vault) {
                Ok(s) => s,
                Err(e) => {
                    println!("error: {}", e);

                    return;
                }
            };

            match secrets.get(&key) {
                Some(secret) => println!("{}", secret.value),
                None => eprintln!("error: secret '{}'  not found in vault '{}'", key, vault),
            }
        }

        Commands::Delete { vault, key } => {
            println!("Deleting {} from vault {}", key, vault);
        }
    }
}
