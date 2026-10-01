use regex::Regex;
use reqwest::Client;
use serde_json::json;

use super::anki::Card;
use super::config::DiscordConfig;

const MAX_WORD_LENGTH: usize = 100;
const MAX_DEFINITION_LENGTH: usize = 500;

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

/*
Anki definitions arrive with every sense concatenated on one line, e.g. "(noun)Definition: ...(verb)Definition: ...", so each sense and example is moved onto its own line.
*/
fn tidy_definition(definition: &str) -> String {
    let sense = Regex::new(r"\s*(?:\d+\.\s*)?(\([^)]+\))?\s*Definition:\s*").unwrap();
    let example = Regex::new(r"\s*Example:\s*").unwrap();
    let with_senses = sense.replace_all(definition, |captures: &regex::Captures| {
        match captures.get(1) {
            Some(pos) => format!("\n{} ", pos.as_str()),
            None => "\n".to_string(),
        }
    });
    example
        .replace_all(&with_senses, "\n  Example: ")
        .trim()
        .to_string()
}

fn format_card(card: &Card) -> String {
    let word = truncate(&card.word, MAX_WORD_LENGTH);
    let definition = truncate(
        &escape_spoiler(&tidy_definition(&card.definition)),
        MAX_DEFINITION_LENGTH,
    );
    format!("**{}**\n||{}||", word, definition)
}

pub fn format_message(cards: &[Card]) -> String {
    let body = cards
        .iter()
        .map(format_card)
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("📖 **Words of the day**\n\n{}", body)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn card(word: &str, definition: &str) -> Card {
        Card {
            word: word.to_string(),
            definition: definition.to_string(),
        }
    }

    #[test]
    fn splits_concatenated_senses_onto_lines() {
        let tidy = tidy_definition(
            "(adjective)Definition: Natural.(verb)Definition: To do. Example: it was done",
        );
        assert_eq!(
            tidy,
            "(adjective) Natural.\n(verb) To do.\n  Example: it was done"
        );
    }

    #[test]
    fn handles_senses_without_a_part_of_speech() {
        assert_eq!(
            tidy_definition("Definition: One.Definition: Two."),
            "One.\nTwo."
        );
    }

    #[test]
    fn normalises_numbered_senses() {
        assert_eq!(
            tidy_definition("1. (noun) Definition: A thing."),
            "(noun) A thing."
        );
    }

    #[test]
    fn wraps_definitions_in_spoilers() {
        let message = format_message(&[card("word", "(noun)Definition: A thing.")]);
        assert!(message.contains("**word**\n||(noun) A thing.||"));
    }

    #[test]
    fn long_messages_keep_spoilers_closed_and_under_the_limit() {
        let long = "x".repeat(5000);
        let cards = vec![card("a", &long), card("b", &long), card("c", &long)];
        let message = format_message(&cards);
        assert!(message.chars().count() <= 2000);
        assert_eq!(message.matches("||").count(), 6);
    }

    #[test]
    fn escapes_spoiler_delimiters_in_definitions() {
        let message = format_message(&[card("w", "a || b")]);
        assert_eq!(message.matches("||").count(), 2);
    }
}
