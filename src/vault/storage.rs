use std::fs;
use std::path::PathBuf;

use crate::models::{Secrets, Vault};

//checking .vaults dir if not there make it
pub fn get_vault_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;

    let vault_dir = home.join(".vaults");

    fs::create_dir_all(&vault_dir)
        .map_err(|e| format!("Failed to create vault directory: {}", e))?;

    Ok(vault_dir)
}

//getting vault path
pub fn get_vault_path(vault_name: &str) -> Result<PathBuf, String> {
    let vault_dir = get_vault_dir()?;
    let vault_path = vault_dir.join(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{}' does not exist", vault_name));
    }

    Ok(vault_path)
}

//creating vault dir
pub fn create_vault_dir(vault_name: &str) -> Result<PathBuf, String> {
    let vault_dir = get_vault_dir()?;
    let vault_path = vault_dir.join(vault_name);

    if vault_path.exists() {
        return Err(format!("Vault '{}' already exists", vault_name));
    }

    fs::create_dir(&vault_path).map_err(|e| format!("Failed to create vault folder: {}", e))?;

    Ok(vault_path)
}

//saving the vault with vault metadata
pub fn save_vault(vault: &Vault) -> Result<(), String> {
    let vault_path = get_vault_path(&vault.name)?;
    let metadata_path = vault_path.join("vault.json");

    let json = serde_json::to_string_pretty(vault)
        .map_err(|e| format!("Failed to serialize vault: {}", e))?;

    fs::write(&metadata_path, json)
        .map_err(|e| format!("failed to write value metadata: {}", e))?;

    Ok(())
}

//loading sercrets from secrets.json
pub fn load_secret(vault_name: &str) -> Result<Secrets, String> {
    let vault_path = get_vault_path(vault_name)?;
    let secret_path = vault_path.join("secrets.json");

    if !secret_path.exists() {
        return Ok(std::collections::HashMap::new());
    }

    let content =
        fs::read_to_string(&secret_path).map_err(|e| format!("failed to read secrets: {}", e))?;

    let secrets: Secrets =
        serde_json::from_str(&content).map_err(|e| format!("failed to parse secrets: {}", e))?;

    Ok(secrets)
}

//saving secrets to secrets.json works
pub fn save_secret(vault_name: &str, secrets: &Secrets) -> Result<(), String> {
    let vault_path = get_vault_path(vault_name)?;
    let secret_path = vault_path.join("secrets.json");

    let json = serde_json::to_string_pretty(secrets)
        .map_err(|e| format!("failed to serialize secrets: {}", e))?;

    fs::write(&secret_path, json).map_err(|e| format!("failed to write secrets: {}", e))?;

    Ok(())
}
