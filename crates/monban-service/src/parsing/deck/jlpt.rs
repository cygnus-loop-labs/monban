use std::path::Path;

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck, DictionaryItem as _, Word, WordCategory};

use crate::{
    parsing::{DeckLoader, ParseError},
    util::load_file,
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
        let content = load_file(file)?;

        let entries: JLPTDeckFile = serde_json::from_str(&content)
            .map_err(|_| ParseError::InvalidFileFormat(name.clone()))?;

        let mut deck = Deck::new();

        for entry in &entries.vocabulary {
            let mut word = Word::new(entry.word.clone(), WordCategory::Unknown);
            word.tag(format!("{}={:?}", &name, entry.level));

            deck.add_word(word);
        }

        Ok(deck)
    }
}
