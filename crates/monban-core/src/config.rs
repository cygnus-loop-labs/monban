use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = include_str!("../../../config.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dictionary: DictionaryConfig,
    pub parser: ParserConfig,
    pub user_decks: DecksConfig,
    pub wk_deck: Option<WKDeckConfig>,
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
    pub filtering: FilteringConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilteringConfig {
    pub include: HashMap<String, HashSet<String>>,
    pub exclude_chars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecksConfig {
    pub decks: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WKDeckConfig {
    pub deck: String,
}
