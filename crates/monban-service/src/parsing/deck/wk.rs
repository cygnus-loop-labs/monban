use std::path::Path;

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck, DictionaryItem, Kanji, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_data_file,
};

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
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Result<Deck, ParseError> {
        let content = load_data_file(file)?;

        let entries: WKDeckFile = serde_json::from_str(&content).unwrap();

        let mut deck = Deck::new();

        for entry in &entries.kanji {
            let mut kanji = Kanji::new(entry.characters.chars().next().unwrap());
            kanji.tag(format!("wk_level={}", entry.level));

            deck.add_kanji(kanji);
        }

        for entry in &entries.vocabulary {
            let deck_entry = deck.add_word(
                entry.characters.clone(),
                entry.readings[0].clone(),
                WordCategory::Unknown,
            );
            deck_entry.tag(format!("{}={}", &name, entry.level));
        }

        Ok(deck)
    }
}
