use monban_core::{Config, Lexicon};
use serde::Serialize;

#[derive(Serialize)]
pub struct Stats {
    word_count: u32,
    unique_word_count: u32,
    kanji_count: u32,
    unique_kanji_count: u32,
}

pub struct WordAnalyzer {}

impl WordAnalyzer {
    pub fn new(_config: &Config) -> Self {
        Self {}
    }

    pub fn analyze(&self, lexicon: &Lexicon) -> Stats {
        Stats {
            word_count: lexicon.words.values().map(|w| w.count).sum(),
            unique_word_count: lexicon.words.len() as u32,
            kanji_count: lexicon.kanji.values().map(|k| k.count).sum(),
            unique_kanji_count: lexicon.kanji.len() as u32,
        }
    }
}
