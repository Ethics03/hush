use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hush")]
#[command(about = "A secure secret management CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        //name of the vault
        name: String,
    },

    Set {
        #[arg(short, long)]
        vault: String,

        #[arg(short, long)]
        key: String,

        #[arg(short = 'V', long)]
        value: String,
    },

    //getting the value of the secret_key
    Get {
        #[arg(short, long)]
        vault: String,

        #[arg(short, long)]
        key: String,
    },

    Delete {
        #[arg(short, long)]
        vault: String,

        #[arg(short, long)]
        key: String,
    },

    List,
}
