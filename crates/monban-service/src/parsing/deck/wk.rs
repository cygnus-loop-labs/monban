use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck};

use crate::parsing::DeckLoader;

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
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Deck {
        let content = fs::read_to_string(file).unwrap();

        let entries: WKDeckFile = serde_json::from_str(&content).unwrap();

        let mut deck = Deck::new();

        for entry in &entries.kanji {
            deck.add_kanji(
                entry.characters.clone(),
                true,
                vec![format!("wk_level={}", entry.level)],
            );
        }

        for entry in &entries.vocabulary {
            deck.add_word(
                entry.characters.clone(),
                true,
                vec![format!("{}={}", &name, entry.level)],
            );
        }

        deck
    }
}
