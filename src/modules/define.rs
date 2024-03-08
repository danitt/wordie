use reqwest::Client;
use serde::Serialize;
use serde_json::{self};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

use crate::r#const::{DEFINITIONS_JSON_PATH, FAILED_WORDS_CSV_PATH, WORDS_CSV_PATH};

use super::typings::Definition;
use super::wordlist::{get_failed_words, get_word_list};

async fn read_existing_definitions() -> io::Result<Vec<Definition>> {
    let path = Path::new(DEFINITIONS_JSON_PATH);
    if !Path::new(path).exists() {
        return Ok(vec![]);
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let definitions = serde_json::from_reader(reader)?;
    Ok(definitions)
}

async fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let json = serde_json::to_string(value)?;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, json)?;
    Ok(())
}

async fn write_text_file(path: &Path, content: &str) -> io::Result<()> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub async fn generate_definitions() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let definitions_path = Path::new(DEFINITIONS_JSON_PATH);
    let failed_words_path = Path::new(FAILED_WORDS_CSV_PATH);
    let mut definitions = read_existing_definitions().await?;
    let mut failed_words = get_failed_words(failed_words_path).await?;
    let defined_words: HashSet<String> =
        definitions.iter().map(|d| d.word.to_lowercase()).collect();
    let all_words = get_word_list(WORDS_CSV_PATH).await?;
    let words: Vec<String> = all_words
        .into_iter()
        .filter(|word| !defined_words.contains(&word.to_lowercase()))
        .collect();

    if words.is_empty() {
        println!("No new words to define");
        return Ok(());
    }

    println!("Found {} new words to define", words.len());
    for word in words {
        println!("Defining word: {}", word);
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
        let resp = client.get(url).send().await?.text().await?;
        let word_definitions: Vec<Definition> = serde_json::from_str(&resp)?;

        if word_definitions.is_empty() {
            println!("Failed to retrieve definition for {}", word);
            failed_words.push(word);
            write_text_file(failed_words_path, &failed_words.join("\n")).await?;
            continue;
        }

        definitions.push(word_definitions[0].clone());
        write_json(definitions_path, &definitions).await?;

        // throttle requests to avoid rate limiting
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}
