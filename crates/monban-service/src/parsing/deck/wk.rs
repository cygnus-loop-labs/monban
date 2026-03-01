use std::path::Path;

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck, DictionaryItem, Kanji, Word, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_file,
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
        let content = load_file(file)?;

        let entries: WKDeckFile = serde_json::from_str(&content).unwrap();

        let mut deck = Deck::new();

        for entry in &entries.kanji {
            let mut kanji = Kanji::new(entry.characters.chars().next().unwrap());
            kanji.tag(format!("wk_level={}", entry.level));

            deck.add_kanji(kanji);
        }

        for entry in &entries.vocabulary {
            let mut word = Word::new(entry.characters.clone(), WordCategory::Unknown);
            word.tag(format!("{}={}", &name, entry.level));
            deck.add_word(word);
        }

        Ok(deck)
    }
}
