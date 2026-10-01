use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
struct HistoryFile {
    used: Vec<String>,
}

pub fn load_history(path: &str) -> HashSet<String> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<HistoryFile>(&raw).ok())
        .map(|file| file.used.into_iter().collect())
        .unwrap_or_default()
}

pub fn save_history(path: &str, used: &HashSet<String>) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut sorted: Vec<String> = used.iter().cloned().collect();
    sorted.sort();
    let json = serde_json::to_string_pretty(&HistoryFile { used: sorted })?;
    fs::write(path, json)
}
