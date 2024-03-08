use std::error::Error;
use std::{fs::File, io::Write, path::Path};

use super::typings::Definition;

pub async fn read_existing_definitions(path: &str) -> Result<Vec<Definition>, Box<dyn Error>> {
    let file = File::open(path)?;
    let definitions: Vec<Definition> = serde_json::from_reader(file)?;
    Ok(definitions)
}

pub async fn generate_anki_cards(
    definitions: Vec<Definition>,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut cards = Vec::new();

    for definition in definitions {
        let front = definition.word;
        let meanings_html = definition
            .meanings
            .iter()
            .enumerate()
            .map(|(_idx, meaning)| {
                meaning
                    .definitions
                    .iter()
                    .map(|def| {
                        let part_of_speech_html = meaning
                            .part_of_speech
                            .as_ref()
                            .map_or(String::new(), |pos| format!("<em>({})</em>", pos));
                        let definition_html =
                            def.definition.as_ref().map_or(String::new(), |definition| {
                                format!("<strong>Definition:</strong> {}", definition)
                            });
                        let example_html = def.example.as_ref().map_or(String::new(), |example| {
                            format!("<strong>Example:</strong> {}", example)
                        });

                        format!(
                            "{}<br />{}<br />{}<br /><br />",
                            part_of_speech_html, definition_html, example_html
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("<br />")
            })
            .collect::<Vec<String>>()
            .join("<br />");
        let back = format!("<div style=\"text-align: left;\">{}</div>", meanings_html);
        let card = format!("{}|{}", front, back).replace("\n", "");
        cards.push(card);
    }

    let mut file = File::create(Path::new(output_path))?;
    write!(file, "{}", cards.join("\n"))?;
    Ok(())
}
