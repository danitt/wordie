use csv::ReaderBuilder;
use std::path::Path;

pub async fn get_word_list(wordlist_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_path = Path::new(wordlist_path);
    if !file_path.exists() {
        return Err("Word list file does not exist.".into());
    }

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    let mut words = vec![];

    for result in rdr.records() {
        let record = result?;
        if let Some(word) = record.get(0) {
            if !word.is_empty() {
                words.push(word.to_string());
            }
        }
    }

    Ok(words)
}

pub async fn get_failed_words(file_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !file_path.exists() {
        return Ok(vec![]);
    }

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    let mut words = vec![];

    for result in rdr.records() {
        let record = result?;
        if let Some(word) = record.get(0) {
            if !word.is_empty() {
                words.push(word.to_string());
            }
        }
    }

    Ok(words)
}
