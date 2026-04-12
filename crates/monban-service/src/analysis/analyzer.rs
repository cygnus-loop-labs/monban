use serde::Serialize;

use monban_core::{Config, DictionaryItem, Lexicon};

#[derive(Default, Serialize)]
pub struct Stats {
    pub words: Count,
    pub kanji: Count,
}

#[derive(Default, Serialize)]
pub struct Count {
    pub count: u32,
    pub unique_count: u32,
    pub unknown_count: u32,
    pub n1_n5_count: (u32, u32, u32, u32, u32),
}

impl Count {
    pub fn new<'a, T: DictionaryItem + 'a>(items: impl Iterator<Item = &'a T>) -> Self {
        let items: Vec<_> = items.into_iter().collect();

        Self {
            count: items.iter().map(|w| w.count()).sum(),
            unique_count: items.len() as u32,
            unknown_count: items.iter().filter(|w| !w.learned()).count() as u32,
            n1_n5_count: (
                items
                    .iter()
                    .filter(|w| w.tags().any(|s| s == "jlpt=N1"))
                    .count() as u32,
                items
                    .iter()
                    .filter(|w| w.tags().any(|s| s == "jlpt=N2"))
                    .count() as u32,
                items
                    .iter()
                    .filter(|w| w.tags().any(|s| s == "jlpt=N3"))
                    .count() as u32,
                items
                    .iter()
                    .filter(|w| w.tags().any(|s| s == "jlpt=N4"))
                    .count() as u32,
                items
                    .iter()
                    .filter(|w| w.tags().any(|s| s == "jlpt=N5"))
                    .count() as u32,
            ),
        }
    }
}

pub struct WordAnalyzer {}

impl WordAnalyzer {
    pub fn new(_config: &Config) -> Self {
        Self {}
    }

    pub fn analyze(&self, lexicon: &Lexicon) -> Stats {
        Stats {
            words: Count::new(lexicon.words.values().filter(|w| !w.filter)),
            kanji: Count::new(lexicon.kanji.values()),
        }
    }
}
