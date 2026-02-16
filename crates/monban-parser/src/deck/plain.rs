use std::{fs, path::Path};

use monban_core::{Config, Deck};

use crate::deck::DeckLoader;

pub struct PlainDeckLoader;

impl DeckLoader for PlainDeckLoader {
    fn load(name: &str, file: impl AsRef<Path>, _config: &Config) -> Deck {
        let mut deck = Deck::new(name);

        let content = fs::read_to_string(file).unwrap();

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            deck.add_word(word, 0, true);
        }

        deck
    }
}
