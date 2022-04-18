use old_norwegian_dictionary::{get_dictionary, DictionaryEntry};
use insta::assert_json_snapshot;

#[test]
fn gets_dictionary() {    
    let result = get_dictionary().to_vec();

    assert_json_snapshot!(result)
}

#[test]
fn exports_dictionary_entry() {    
    let entry = DictionaryEntry {
        word: &"Lorem ipsum",
        part_of_speech: &"Dolor sit igitur",
        definition: &"Dolor sit amet",        
    };

    assert_eq!(entry.word, "Lorem ipsum");
    assert_eq!(entry.definition, "Dolor sit amet");
    assert_eq!(entry.part_of_speech, "Dolor sit igitur");
}