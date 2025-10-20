use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod crypto;
mod models;
mod vault;
use anyhow::{Context, Result};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose, Engine as _};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => create_vault(&name),
        Commands::Set { vault, key, value } => set_secret(&vault, &key, &value),
        Commands::Get { vault, key } => get_secret(&vault, &key),
        Commands::Delete { vault, key } => delete_secret(&vault, &key),
        Commands::List => list_vaults(),
    }
}

fn create_vault(name: &str) -> Result<()> {
    vault::create_vault_dir(name)?;

    let salt = SaltString::generate(&mut OsRng);
    let salt_base64 = general_purpose::STANDARD.encode(salt.as_ref());

    let _password = {
        let mut attempts = 0;
        loop {
            let pass1 = rpassword::prompt_password("Enter master password: ")?;
            let pass2 = rpassword::prompt_password("Confirm master password: ")?;

            if pass1 == pass2 {
                if pass1.len() < 8 {
                    println!("Password must be atleast 8 characters\n");
                } else {
                    break pass1;
                }
            } else {
                println!("Passwords do not match\n")
            }
            attempts += 1;
            if attempts >= 3 {
                anyhow::bail!("Too many failed attempts. Please try again.");
            }
        }
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let vault_metadata = models::Vault {
        name: name.to_string(),
        created_at: timestamp,
        updated_at: timestamp,
        salt: salt_base64,
        nonce: None,
    };

    vault::save_vault(&vault_metadata)?;

    println!("Vault '{}' created successfully", name);
    Ok(())
}

fn list_vaults() -> Result<()> {
    let vaults = vault::list_vaults()?;

    if vaults.is_empty() {
        println!("No vault found. Create one with: hush create <vault name>");
        return Ok(());
    }

    println!("Available Vaults");
    for vault_name in vaults {
        println!("  • {}", vault_name);
    }

    Ok(())
}

fn set_secret(vault_name: &str, key: &str, value: &str) -> Result<()> {
    let mut vault_metadata = vault::load_vault(vault_name)?;
    let encryption_key = vault::get_encryption_key(&vault_metadata)?;

    // Load existing secrets
    let mut secrets = vault::decrypt_secrets(vault_name, &vault_metadata, &encryption_key)?;

    // Add new secret
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let secret = models::Secret {
        key: key.to_string(),
        value: value.to_string(),
        created_at: timestamp,
    };

    secrets.insert(key.to_string(), secret);

    // again encrypting with nonce
    let json_bytes = serde_json::to_vec(&secrets).context("Failed to serialize secrets")?;

    let (ciphertext, nonce) = crypto::encrypt(&json_bytes, &encryption_key)?;

    vault::save_encrypted_secrets(vault_name, &ciphertext)?;

    // update metadata
    vault_metadata.nonce = Some(general_purpose::STANDARD.encode(&nonce));
    vault_metadata.updated_at = timestamp;
    vault::save_vault(&vault_metadata)?;

    println!(
        "Secret '{}' saved to vault '{}' (encrypted)",
        key, vault_name
    );
    Ok(())
}

fn get_secret(vault_name: &str, key: &str) -> Result<()> {
    let vault_metadata = vault::load_vault(vault_name)?;
    let encryption_key = vault::get_encryption_key(&vault_metadata)?;

    let secrets = vault::decrypt_secrets(vault_name, &vault_metadata, &encryption_key)?;

    match secrets.get(key) {
        Some(secret) => println!("{}", secret.value),
        None => anyhow::bail!("Secret '{}' not found in vault '{}'", key, vault_name),
    }

    Ok(())
}

fn delete_secret(vault_name: &str, key: &str) -> Result<()> {
    let mut vault_metadata = vault::load_vault(vault_name)?;
    let encryption_key = vault::get_encryption_key(&vault_metadata)?;

    let mut secrets = vault::decrypt_secrets(vault_name, &vault_metadata, &encryption_key)?;

    if secrets.remove(key).is_none() {
        anyhow::bail!("Secret '{}' not found in vault '{}'", key, vault_name);
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let json_bytes = serde_json::to_vec(&secrets).context("Failed to serialize secrets")?;

    let (ciphertext, nonce) = crypto::encrypt(&json_bytes, &encryption_key)?;

    vault::save_encrypted_secrets(vault_name, &ciphertext)?;

    // update metadata
    vault_metadata.nonce = Some(general_purpose::STANDARD.encode(&nonce));
    vault_metadata.updated_at = timestamp;
    vault::save_vault(&vault_metadata)?;

    println!("Secret '{}' deleted from vault '{}'", key, vault_name);
    Ok(())
}

