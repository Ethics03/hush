use clap::Parser;

mod cli;
mod commands;
mod crypto;
mod models;
mod vault;

use anyhow::Result;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => commands::create_vault(&name),
        Commands::Set { vault, key, value } => commands::set_secret(&vault, &key, value),
        Commands::Get { vault, key } => commands::get_secret(&vault, &key),
        Commands::Delete { vault, key } => commands::delete_secret(&vault, &key),
        Commands::List => commands::list_vaults(),
        Commands::DeleteVault { name } => commands::delete_vault(&name),
    }
}
