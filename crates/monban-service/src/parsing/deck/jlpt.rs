use std::path::Path;

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck, DictionaryItem as _, Word, WordCategory};

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
            let mut word = Word::new(entry.word.clone(), WordCategory::Unknown);
            word.tag(format!("{}={:?}", &name, entry.level));
            match entry.level {
                JLPTLevel::N1 => word.learned = false,
                JLPTLevel::N2 => word.learned = false,
                JLPTLevel::N3 => word.learned = false,
                JLPTLevel::N4 => word.learned = true,
                JLPTLevel::N5 => word.learned = true,
            }

            deck.add_word(word);
        }

        Ok(deck)
    }
}
