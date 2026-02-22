use std::{collections::HashSet, fs, path::Path};

use serde::{Deserialize, Serialize};

use monban_core::{Config, Deck};

use crate::parsing::DeckLoader;

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
    fn load(name: String, file: impl AsRef<Path>, _config: &Config) -> Deck {
        let content = fs::read_to_string(file).unwrap();

        let entries: JLPTDeckFile = serde_json::from_str(&content).unwrap();

        let mut deck = Deck::new();

        for entry in &entries.vocabulary {
            deck.add_word(
                entry.word.clone(),
                true,
                HashSet::from([format!("{}={:?}", &name, entry.level)]),
            );
        }

        deck
    }
}
