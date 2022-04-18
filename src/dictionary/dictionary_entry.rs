use serde::{Deserialize, Serialize};

/// Individual dictionary entry.
/// Each entry contains a headword, definition and type of the word.
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct DictionaryEntry {
	pub word: &'static str,
	pub part_of_speech: &'static str,
	pub definition: &'static str,
}
