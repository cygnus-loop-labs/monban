use std::{fs, path::Path};

use monban_core::Deck;

pub struct PlainDeckLoader;

impl PlainDeckLoader {
    pub fn load(file: impl AsRef<Path>) -> Deck {
        let mut deck = Deck::default();

        let content = fs::read_to_string(file).unwrap();

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            deck.words.insert(word);
        }

        deck
    }
}
