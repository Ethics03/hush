use clap::{Parser, Subcommand};

const ASCII_ART: &str = r#"
 ██░ ██  █    ██   ██████  ██░ ██ 
▓██░ ██▒ ██  ▓██▒▒██    ▒ ▓██░ ██▒
▒██▀▀██░▓██  ▒██░░ ▓██▄   ▒██▀▀██░
░▓█ ░██ ▓▓█  ░██░  ▒   ██▒░▓█ ░██ 
░▓█▒░██▓▒▒█████▓ ▒██████▒▒░▓█▒░██▓
 ▒ ░░▒░▒░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒
 ▒ ░▒░ ░░░▒░ ░ ░ ░ ░▒  ░ ░ ▒ ░▒░ ░
 ░  ░░ ░ ░░░ ░ ░ ░  ░  ░   ░  ░░ ░
 ░  ░  ░   ░           ░   ░  ░  ░
                                  

"#;

#[derive(Parser)]
#[command(name = "hush")]
#[command(version = "1.0.0")]
#[command(about = format!("{}\nA secure and encrypted secret/password manager CLI",ASCII_ART),long_about = None)]
#[command(author = "Rachit Srivastava")]
#[command(styles = get_styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Create a new vault
    Create {
        ///Name of the vault
        name: String,
    },

    ///Store a secret (encrypted)
    Set {
        #[arg(short, long, help = "Name of the vault")]
        #[arg(short, long)]
        vault: String,

        #[arg(short, long, help = "Secret key/name")]
        key: String,

        #[arg(short = 'V', long, help = "Secret value to store")]
        value: Option<String>,
    },

    ///Get a secret from the vault
    Get {
        #[arg(short, long, help = "Name of the vault")]
        vault: String,

        #[arg(short, long, help = "Secret key to retrieve")]
        key: String,
    },

    ///Delete a secret
    Delete {
        #[arg(short, long, help = "Name of the vault")]
        vault: String,

        #[arg(short, long, help = "Secret key to delete")]
        key: String,
    },

    ///Delete the entire vault
    DeleteVault {
        ///Name of the vault to delete
        name: String,
    },

    List,
}

fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::AnsiColor;

    clap::builder::Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
}
