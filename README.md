# Old Norwegian Dictionary

Old Norwegian/Norse Dictionary for Rust. The dictionary consists of 40 000+ Old Norse words with Norwegian translations.

Based on "Dictionary of the Old Norwegian Language".

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
old_norwegian_dictionary = "1.0.0"
```

### Usage

```rust
use old_norwegian_dictionary::{get_dictionary, DictionaryEntry};

// Whole dictionary as a vector of DictionaryEntries.
let dictionary = get_dictionary();

// Returns a Result, which should always be safe to unwrap.
// Up to you if you wish to just unwrap, or use other error handling method.
let dictionary_content: Vec<DictionaryEntry> = dictionary.unwrap();

println!("A word from dictionary: {}. Its definition is: {}. Its type if {}", &dictionary_content[0].word, &dictionary_content[0].definition, &dictionary_content[0].part_of_speech)
```

### About "Dictionary of the Old Norwegian Language"

_"Ordbog over det gamle norske Sprog"_ dictionary was published in late 1800s by Johan Fritzner. Its is the largest Old Norse to Norwegian dictionary, containing over 40 000 word definitions. While the original dictionary is called dictionary of "old norwegian", it is practically a dictionary of western Old Norse. Technically "Old Norwegian" would be a later stage in the language.
