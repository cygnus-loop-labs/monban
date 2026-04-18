use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::WordCategory;

const DEFAULT_CONFIG: &str = include_str!("../../../config/config.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub data_dir: PathBuf,
    pub dictionary: DictionaryConfig,
    pub parser: ParserConfig,
    pub decks: Vec<DeckConfig>,
    pub anki: AnkiConfig,
}

impl Config {
    pub fn load() -> Self {
        let config: Self = toml::from_str(DEFAULT_CONFIG).expect("Failed to parse config");

        config
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryConfig {
    pub words: String,
    pub kanji: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParserConfig {
    pub dictionary: String,
    pub blacklist: String,
    pub filtering: FilteringConfig,
    pub mapping: HashMap<WordCategory, Vec<(String, String)>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilteringConfig {
    pub exclude_chars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeckConfig {
    Anki {
        name: String,
        word: String,
        reading: String,
        meaning: String,
    },
    File {
        name: String,
        path: String,
    },
    Jlpt {
        name: String,
        path: String,
    },
    Wk {
        name: String,
        path: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiConfig {
    pub url: String,
}
