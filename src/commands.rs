use std::io::{self, Write};

use anyhow::{Context, Result};
use arboard::Clipboard;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose, Engine as _};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{crypto, models, vault};

pub fn create_vault(name: &str) -> Result<()> {
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

pub fn list_vaults() -> Result<()> {
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

pub fn set_secret(vault_name: &str, key: &str, value: Option<String>) -> Result<()> {
    let mut vault_metadata = vault::load_vault(vault_name)?;
    let encryption_key = vault::get_encryption_key(&vault_metadata)?;

    let mut secrets = vault::decrypt_secrets(vault_name, &vault_metadata, &encryption_key)?;

    let secret_value = match value {
        Some(v) => v,
        None => rpassword::prompt_password("Enter secret value: ")?,
    };
    // Add new secret
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let secret = models::Secret {
        key: key.to_string(),
        value: secret_value,
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

pub fn get_secret(vault_name: &str, key: &str) -> Result<()> {
    let vault_metadata = vault::load_vault(vault_name)?;
    let encryption_key = vault::get_encryption_key(&vault_metadata)?;

    let secrets = vault::decrypt_secrets(vault_name, &vault_metadata, &encryption_key)?;

    match secrets.get(key) {
        Some(secret) => {
            match Clipboard::new() {
                Ok(mut clipboard) => match clipboard.set_text(&secret.value) {
                    Ok(_) => {
                        println!("Secret '{}' copied to clipboard", key);
                        println!("Paste it anywhere you need to!");

                        //this is for linux clipboard guy closed too fast
                        thread::sleep(Duration::from_millis(300));
                    }
                    Err(_) => {
                        println!("Clipboard unavailable. Here's your secret: ");
                        println!("{}", secret.value);
                    }
                },
                Err(_) => {
                    //fallback -> if clipbard exists bro
                    println!("Clipboard unavailable. Here's your secret: ");
                    println!("{}", secret.value);
                }
            }
        }
        None => anyhow::bail!("Secret '{}' not found in vault '{}'", key, vault_name),
    }

    Ok(())
}

pub fn delete_secret(vault_name: &str, key: &str) -> Result<()> {
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

pub fn delete_vault(vault_name: &str) -> Result<()> {
    let _ = vault::load_vault(vault_name)?;

    print!("Delete vault '{}' and ALL its secrets? [y/N]: ", vault_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        vault::delete_vault(vault_name)?;
        println!("Vault '{}' deleted permanently", vault_name);
    } else {
        println!("Deletion cancelled");
    }

    Ok(())
}
