use crate::modules::anki::select_cards;
use crate::modules::config::{anki_config, discord_config, history_path, load_env_file};
use crate::modules::discord::{format_message, post_message};
use crate::modules::wod_history::{load_history, save_history};

const WORD_COUNT: usize = 3;

async fn execute(dry_run: bool) -> Result<(), String> {
    load_env_file();
    let anki = anki_config()?;
    let discord = if dry_run {
        None
    } else {
        Some(discord_config()?)
    };

    let path = history_path();
    let mut used = load_history(&path);
    let selection = select_cards(&anki, &used, WORD_COUNT)
        .await
        .map_err(|e| e.to_string())?;
    if selection.history_reset {
        println!("Not enough unused words found, resetting history");
        used.clear();
    }
    let message = format_message(&selection.cards);

    if let Some(discord) = discord {
        post_message(&discord, &message).await?;
        used.extend(selection.cards.iter().map(|card| card.word.clone()));
        save_history(&path, &used).map_err(|e| e.to_string())?;
        println!("⚡ Posted {} words to Discord", selection.cards.len());
    } else {
        println!("{}", message);
    }
    Ok(())
}

pub async fn run(dry_run: bool) {
    if let Err(e) = execute(dry_run).await {
        eprintln!("💥 Error running word of the day: {}", e);
        std::process::exit(1);
    }
}
