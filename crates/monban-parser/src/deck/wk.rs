use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use monban_core::Deck;

use crate::deck::DeckLoader;

#[derive(Serialize, Deserialize)]
struct WKEntry {
    id: u32,
    characters: String,
    meanings: Vec<String>,
    readings: Vec<String>,
    level: u32,
}

#[derive(Serialize, Deserialize)]
struct WKDeckFile {
    pub kanji: Vec<WKEntry>,
    pub vocabulary: Vec<WKEntry>,
}

pub struct WKDeckLoader;

impl DeckLoader for WKDeckLoader {
    fn load(name: &str, file: impl AsRef<Path>) -> Deck {
        let content = fs::read_to_string(file).unwrap();

        let entries: WKDeckFile = serde_json::from_str(&content).unwrap();

        let mut deck = Deck::new(name);

        for entry in &entries.kanji {
            deck.add_kanji(entry.characters.clone(), entry.level);
        }

        for entry in &entries.vocabulary {
            deck.add_word(entry.characters.clone(), entry.level);
        }

        deck
    }
}
