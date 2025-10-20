use anyhow::{Context, Result};
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

use crate::crypto;
use crate::models::{Secret, Vault};
use crate::vault;

//getting the encryption key for each time we put password
pub fn get_encryption_key(vault_metadata: &Vault) -> Result<[u8; 32]> {
    let password = rpassword::prompt_password("Enter master password: ")?;

    let salt_bytes = general_purpose::STANDARD
        .decode(&vault_metadata.salt)
        .context("Failed to decode salt")?;

    let salt_str = std::str::from_utf8(&salt_bytes).context("invalid salt format")?;

    let salt = SaltString::from_b64(&salt_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse salt: {}", e))?;

    crypto::derive_key(&password, &salt)
}

pub fn decrypt_secrets(
    vault_name: &str,
    vault_metadata: &Vault,
    encryption_key: &[u8],
) -> Result<HashMap<String, Secret>> {
    let encrypted_data = vault::load_encrypted_secrets(vault_name)?;

    if encrypted_data.is_empty() {
        return Ok(std::collections::HashMap::new());
    }

    let nonce_base64 = vault_metadata
        .nonce
        .as_ref()
        .context("nonce not found in vault metadata")?;

    let nonce = general_purpose::STANDARD
        .decode(nonce_base64)
        .context("failed to decode nonce")?;

    let decrypted = crypto::decrypt(&encrypted_data, &encryption_key, &nonce)?;

    let secrets: HashMap<String, Secret> =
        serde_json::from_slice(&decrypted).context("failed to parse decrypted secrets")?;

    Ok(secrets)
}
