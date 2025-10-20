use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::Vault;

//checking .vaults dir if not there make it
pub fn get_vault_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    let vault_dir = home.join(".vaults");

    fs::create_dir_all(&vault_dir).context("Failed to create vault directory")?;

    Ok(vault_dir)
}

//getting vault path
pub fn get_vault_path(vault_name: &str) -> Result<PathBuf> {
    let vault_dir = get_vault_dir()?;
    let vault_path = vault_dir.join(vault_name);

    if !vault_path.exists() {
        anyhow::bail!("Vault '{}' does not exist", vault_name);
    }

    Ok(vault_path)
}

//creating vault dir
pub fn create_vault_dir(vault_name: &str) -> Result<PathBuf> {
    let vault_dir = get_vault_dir()?;
    let vault_path = vault_dir.join(vault_name);

    if vault_path.exists() {
        anyhow::bail!("Vault '{}' already exists", vault_name);
    }

    fs::create_dir(&vault_path).context("Failed to create vault folder")?;

    Ok(vault_path)
}

//saving the vault with vault metadata
pub fn save_vault(vault: &Vault) -> Result<()> {
    let vault_path = get_vault_path(&vault.name)?;
    let metadata_path = vault_path.join("vault.json");

    let json = serde_json::to_string_pretty(vault).context("Failed to serialize vault")?;

    fs::write(&metadata_path, json).context("Failed to write vault metadata")?;

    Ok(())
}

pub fn load_vault(vault_name: &str) -> Result<Vault> {
    let vault_path = get_vault_path(vault_name)?;

    let metadata_path = vault_path.join("vault.json");

    let content = fs::read_to_string(&metadata_path).context("Failed to read vault metadata")?;

    let vault: Vault = serde_json::from_str(&content).context("failed to parse vault metadata")?;

    Ok(vault)
}

pub fn save_encrypted_secrets(vault_name: &str, encrypted_data: &[u8]) -> Result<()> {
    let vault_path = get_vault_path(vault_name)?;
    let secret_path = vault_path.join("secrets.enc");

    fs::write(&secret_path, encrypted_data).context("Failed to write encrypted data")?;

    Ok(())
}

pub fn load_encrypted_secrets(vault_name: &str) -> Result<Vec<u8>> {
    let vault_path = get_vault_path(vault_name)?;
    let secret_path = vault_path.join("secrets.enc");

    if !secret_path.exists() {
        return Ok(Vec::new());
    }

    fs::read(&secret_path).context("failed to read encrypted secrets")
}

pub fn list_vaults() -> Result<Vec<String>> {
    let vault_dir = get_vault_dir()?;

    let mut vaults = Vec::new();

    for entry in fs::read_dir(&vault_dir).context("failed to read vault directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    vaults.push(name_str.to_string());
                }
            }
        }
    }

    vaults.sort();

    Ok(vaults)
}
