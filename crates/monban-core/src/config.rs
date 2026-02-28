use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::WordCategory;

const DEFAULT_CONFIG: &str = include_str!("../../../config/config.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dictionary: DictionaryConfig,
    pub parser: ParserConfig,
    pub decks: HashMap<String, DeckConfig>,
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
    pub mapping: HashMap<WordCategory, Vec<(String, String)>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilteringConfig {
    pub exclude_chars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckConfig {
    pub file: String,
    #[serde(rename = "type")]
    pub ty: String,
}
