use old_norwegian_dictionary::{get_dictionary, DictionaryEntry};
use insta::assert_json_snapshot;

#[test]
fn gets_dictionary() {    
    let result = get_dictionary().unwrap();

    assert_json_snapshot!(result)
}

#[test]
fn exports_dictionary_entry() {    
    let entry = DictionaryEntry {
        word: "Lorem ipsum".to_string(),
        part_of_speech: "Dolor sit igitur".to_string(),
        definition: "Dolor sit amet".to_string(),        
    };

    assert_eq!(entry.word, "Lorem ipsum");
    assert_eq!(entry.definition, "Dolor sit amet");
    assert_eq!(entry.part_of_speech, "Dolor sit igitur");
}