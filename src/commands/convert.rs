use crate::modules::convert::{generate_anki_cards, read_existing_definitions};

pub async fn run() {
    let definitions = read_existing_definitions("./output/definitions.json")
        .await
        .unwrap();
    match generate_anki_cards(definitions, "./output/anki.txt").await {
        Ok(_) => println!("⚡ Converted to Anki!"),
        Err(e) => eprintln!("💥 Error generating definitions: {}", e),
    }
}
