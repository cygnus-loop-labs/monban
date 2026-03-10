use std::path::Path;

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_data_file,
};

#[derive(Debug, Serialize, Deserialize)]
enum JLPTLevel {
    N1,
    N2,
    N3,
    N4,
    N5,
}

#[derive(Serialize, Deserialize)]
struct JLPTEntry {
    word: String,
    meaning: String,
    reading: String,
    level: JLPTLevel,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct JLPTDeckFile {
    pub vocabulary: Vec<JLPTEntry>,
}

pub struct JLPTDeckLoader;

impl DeckLoader for JLPTDeckLoader {
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Result<Deck, ParseError> {
        let content = load_data_file(file)?;

        let entries: JLPTDeckFile = serde_json::from_str(&content)
            .map_err(|_| ParseError::InvalidFileFormat(name.clone()))?;

        let mut deck = Deck::new();

        for entry in &entries.vocabulary {
            let deck_entry = deck.add_word(
                entry.word.clone(),
                entry.reading.clone(),
                WordCategory::Unknown,
            );
            deck_entry.tag(format!("{}={:?}", &name, entry.level));
            match entry.level {
                JLPTLevel::N1 => deck_entry.learned = false,
                JLPTLevel::N2 => deck_entry.learned = false,
                JLPTLevel::N3 => deck_entry.learned = false,
                JLPTLevel::N4 => deck_entry.learned = true,
                JLPTLevel::N5 => deck_entry.learned = true,
            }
        }

        Ok(deck)
    }
}
