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

#[derive(Deserialize, Debug)]
pub struct DictionaryResponse {
    pub word: String,
    #[serde(default)]
    pub entries: Vec<DictionaryEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryEntry {
    pub part_of_speech: Option<String>,
    #[serde(default)]
    pub pronunciations: Vec<DictionaryPronunciation>,
    #[serde(default)]
    pub senses: Vec<DictionarySense>,
}

#[derive(Deserialize, Debug)]
pub struct DictionaryPronunciation {
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DictionarySense {
    pub definition: Option<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub synonyms: Vec<String>,
}

impl From<DictionaryResponse> for Definition {
    fn from(response: DictionaryResponse) -> Self {
        let phonetics = response
            .entries
            .iter()
            .flat_map(|entry| entry.pronunciations.iter())
            .map(|pronunciation| Phonetic {
                text: pronunciation.text.clone(),
                audio: None,
            })
            .collect();

        let meanings = response
            .entries
            .into_iter()
            .map(|entry| Meaning {
                part_of_speech: entry.part_of_speech,
                definitions: entry
                    .senses
                    .into_iter()
                    .map(DefinitionDetail::from)
                    .collect(),
            })
            .collect();

        Definition {
            word: response.word,
            phonetics,
            meanings,
        }
    }
}

impl From<DictionarySense> for DefinitionDetail {
    fn from(sense: DictionarySense) -> Self {
        DefinitionDetail {
            definition: sense.definition,
            example: sense.examples.into_iter().next(),
            synonyms: sense.synonyms,
        }
    }
}
