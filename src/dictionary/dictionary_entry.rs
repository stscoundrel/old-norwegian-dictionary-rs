use serde::{Deserialize, Serialize};

/// Individual dictionary entry.
/// Each entry contains a headword, definition and type of the word.
#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    #[serde(rename = "partOfSpeech")] // Remap to snake_case key
    pub part_of_speech: String,
    pub definition: String,
}
