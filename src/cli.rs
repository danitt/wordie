use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Generate definitions for wordlist")]
    Define,
    #[command(about = "Convert to Anki format")]
    Convert,
}

pub async fn prompt() {
    let args = Cli::parse();

    match args.command {
        Commands::Define => {
            commands::define::run().await;
        }
        Commands::Convert => {
            commands::convert::run().await;
        }
    }
}
