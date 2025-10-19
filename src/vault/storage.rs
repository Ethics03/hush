use std::fs;
use std::path::PathBuf;

pub fn get_vault_dir() -> Result<PathBuf, String> {
    //getting the home dir
    let home = dirs::home_dir().ok_or("count not find home directory".to_string())?;
    let vault = home.join(".vault");

    //if vault exists prompt else create it
    if !vault.exists() {
        fs::create_dir_all(&vault)
            .map_err(|e| format!("Failed to create vault directory: {:?}: {}", vault, e))?;
        println!("Vault created: {:?}", vault);
    } else {
        println!("vault already exists: {:?}", vault);
    }

    Ok(vault)
}
