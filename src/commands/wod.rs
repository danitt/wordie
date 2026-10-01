use rand::seq::IndexedRandom;
use std::collections::HashSet;

use crate::modules::anki::{fetch_cards, Card};
use crate::modules::config::{anki_config, discord_config, history_path, load_env_file};
use crate::modules::discord::{format_message, post_message};
use crate::modules::wod_history::{load_history, save_history};

const WORD_COUNT: usize = 3;

fn pick_words(cards: &[Card], used: &mut HashSet<String>) -> Vec<Card> {
    let mut available: Vec<&Card> = cards.iter().filter(|c| !used.contains(&c.word)).collect();
    if available.len() < WORD_COUNT {
        println!(
            "Only {} unused words remaining, resetting history",
            available.len()
        );
        used.clear();
        available = cards.iter().collect();
    }
    available
        .sample(&mut rand::rng(), WORD_COUNT)
        .map(|card| (*card).clone())
        .collect()
}

async fn execute(dry_run: bool) -> Result<(), String> {
    load_env_file();
    let anki = anki_config()?;
    let discord = if dry_run {
        None
    } else {
        Some(discord_config()?)
    };

    let cards = fetch_cards(&anki).await.map_err(|e| e.to_string())?;
    if cards.is_empty() {
        return Err("No cards found in the Anki deck".to_string());
    }
    println!("Found {} cards", cards.len());

    let path = history_path();
    let mut used = load_history(&path);
    let selected = pick_words(&cards, &mut used);
    let message = format_message(&selected);

    if let Some(discord) = discord {
        post_message(&discord, &message).await?;
        used.extend(selected.iter().map(|card| card.word.clone()));
        save_history(&path, &used).map_err(|e| e.to_string())?;
        println!("⚡ Posted {} words to Discord", selected.len());
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
