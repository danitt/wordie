use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionFile {
    pub definitions: Vec<Definition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Definition {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phonetic {
    // Phonetic pronunciation
    pub text: Option<String>,
    // Path to audio file
    pub audio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meaning {
    pub part_of_speech: Option<String>,
    pub definitions: Vec<DefinitionDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefinitionDetail {
    pub definition: Option<String>,
    pub example: Option<String>,
    pub synonyms: Vec<String>,
}
