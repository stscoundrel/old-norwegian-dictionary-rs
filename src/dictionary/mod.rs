mod dictionary_entry;
pub use dictionary_entry::DictionaryEntry;
use crate::reader;

const DICTIONARY_PATH: &str = "src/dictionary/dataset/dictionary.json";


/// Get full list of dictionary words.
/// 
/// The Result should always be safe to unwrap.
/// Up to you if you wish to just unwrap, or wrap it in error handling.
/// 
/// # Examples
/// 
/// ```
/// use old_norwegian_dictionary::{get_dictionary, DictionaryEntry};
/// 
/// // Returns a Result, which should always be safe to unwrap.
/// // Up to you if you wish to just unwrap, or use other error handling method.
/// let dictionary: Vec<DictionaryEntry> = get_dictionary().unwrap();
/// 
/// println!("First word is {}, definition for it being {}", &dictionary[0].word, &dictionary[0].definition)
/// ```
pub fn get_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    let contents = reader::read_json_file(DICTIONARY_PATH).unwrap();

    match serde_json::from_str(&contents){
        Ok(entries) => Ok(entries),
        Err(_e) => Err("Failed to serialize dictionary to DictionaryEntries"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_expected_entry_content() {
        let result = get_dictionary();
        let dictionary = result.unwrap();
        let entry: &DictionaryEntry = &dictionary[1989];

        assert_eq!(entry.word, "árgali");
        assert_eq!(entry.definition, "árgali, m. den tidligt galende, = hani.Fm. VI, 251.");
        assert_eq!(entry.part_of_speech, "m");
    }

    #[test]
    fn dictionary_has_42021_entries() {
        let result = get_dictionary();
        let dictionary = result.unwrap();

        assert_eq!(dictionary.len(), 42021);
    }
}