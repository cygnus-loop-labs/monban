use std::path::Path;

use monban_core::{Config, Deck, DictionaryItem as _, Word, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_file,
};

pub struct PlainDeckLoader;

impl DeckLoader for PlainDeckLoader {
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Result<Deck, ParseError> {
        let mut deck = Deck::new();

        let content = load_file(file)?;

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            let mut word = Word::new(word, WordCategory::Unknown);
            word.tag(name.clone());

            deck.add_word(word);
        }

        Ok(deck)
    }
}
