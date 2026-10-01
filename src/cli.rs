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
    #[command(about = "Post three Anki words of the day to Discord")]
    Wod {
        #[arg(long, help = "Print the message instead of posting it")]
        dryrun: bool,
    },
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
        Commands::Wod { dryrun } => {
            commands::wod::run(dryrun).await;
        }
    }
}
