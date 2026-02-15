use std::{fs, path::PathBuf};

use monban_core::Deck;

pub struct PlainDeckLoader;

impl PlainDeckLoader {
    pub fn load(file: impl Into<PathBuf>) -> Deck {
        let mut deck = Deck::default();

        let content = fs::read_to_string(file.into()).unwrap();

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            deck.words.insert(word);
        }

        deck
    }
}
