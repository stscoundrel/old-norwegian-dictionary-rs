//! Old Norwegian/Norse Dictionary for Rust.
//! 
//! The dictionary consists of 40 000+ Old Norse words with Norwegian translations.
//! 
//! Based on "Dictionary of the Old Norwegian Language".

mod dictionary;
pub use dictionary::{get_dictionary, DictionaryEntry};