use std::path::Path;

use monban_core::{Config, Deck, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_data_file,
};

pub struct PlainDeckLoader;

impl DeckLoader for PlainDeckLoader {
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Result<Deck, ParseError> {
        let mut deck = Deck::new(&name);

        let content = load_data_file(file)?;

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            let deck_entry =
                deck.add_word(word.clone(), word, "".to_string(), WordCategory::Unknown);
            deck_entry.tag(name.clone());
            deck_entry.learned = true;
        }

        Ok(deck)
    }
}
