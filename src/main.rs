use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod models;

use crate::vault::get_vault_dir;
mod vault;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => match get_vault_dir() {
            Ok(home_vault_path) => {
                let vault_name = name.clone().unwrap_or_else(|| "default".to_string());
                let vault_path = home_vault_path.join(&vault_name);

                if !vault_path.exists() {
                    std::fs::create_dir_all(&vault_path)
                        .expect("Failed to create named vault folder");
                    println!("Vault '{}' created at {:?}", vault_name, vault_path);
                } else {
                    println!("Vault '{}' already exists at {:?}", vault_name, vault_path);
                }
            }
            Err(err) => eprintln!("Error accessing home vault: {}", err),
        },
        Commands::Set { vault, key, value } => {
            println!("setting {}={} in vault {}", key, value, vault);
        }

        Commands::Get { vault, key } => {
            println!("Getting {} from vault {}", key, vault);
        }

        Commands::Delete { vault, key } => {
            println!("Deleting {} from vault {}", key, vault);
        }
    }
}
