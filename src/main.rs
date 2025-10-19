use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod crypto;
mod models;
mod vault;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose, Engine as _};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => {
            // Create vault directory
            vault::create_vault_dir(&name)?;

            let salt = SaltString::generate(&mut OsRng);
            let salt_base64 = general_purpose::STANDARD.encode(salt.as_ref());

            let _password = rpassword::prompt_password("Enter master password: ")?;

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
                salt: salt_base64,
            };

            // saving vault
            vault::save_vault(&vault)?;
            // creating empty secrets.json
            let empty_secrets = std::collections::HashMap::new();
            vault::save_secret(&name, &empty_secrets)?;

            println!("Vault '{}' created successfully", name);
        }

        Commands::Set { vault, key, value } => {
            let mut secrets = vault::load_secret(&vault)?;

            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

            let secret = models::Secret {
                key: key.clone(),
                value: value.clone(),
                created_at: timestamp,
            };

            //insert into hashmap
            secrets.insert(key.clone(), secret);

            vault::save_secret(&vault, &secrets)?;
        }
        //loadng the secrets from the vault
        Commands::Get { vault, key } => {
            let secrets = vault::load_secret(&vault)?;

            match secrets.get(&key) {
                Some(secret) => println!("{}", secret.value),
                None => eprintln!("error: secret '{}'  not found in vault '{}'", key, vault),
            }
        }

        Commands::Delete { vault, key } => {
            println!("Deleting {} from vault {}", key, vault);
        }
    }

    Ok(())
}
