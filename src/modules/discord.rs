use reqwest::Client;
use serde_json::json;

use super::anki::Card;
use super::config::DiscordConfig;

const MESSAGE_LIMIT: usize = 2000;
const MAX_DEFINITION_LENGTH: usize = 400;

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let kept: String = text.chars().take(max_chars).collect();
    format!("{}…", kept.trim_end())
}

fn escape_spoiler(text: &str) -> String {
    text.replace("||", "| |")
}

fn format_card(card: &Card) -> String {
    let definition = truncate(&escape_spoiler(&card.definition), MAX_DEFINITION_LENGTH);
    format!("**{}**\n||{}||", card.word, definition)
}

pub fn format_message(cards: &[Card]) -> String {
    let body = cards
        .iter()
        .map(format_card)
        .collect::<Vec<_>>()
        .join("\n\n");
    let message = format!("📖 **Words of the day**\n\n{}", body);
    truncate(&message, MESSAGE_LIMIT)
}

pub async fn post_message(config: &DiscordConfig, content: &str) -> Result<(), String> {
    let url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        config.channel_id
    );
    let response = Client::new()
        .post(url)
        .header("Authorization", format!("Bot {}", config.token))
        .json(&json!({ "content": content }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.status().is_success() {
        return Ok(());
    }
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    Err(format!("Discord responded {}: {}", status, body))
}
