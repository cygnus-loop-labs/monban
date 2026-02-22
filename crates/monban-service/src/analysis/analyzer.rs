use monban_core::{Config, Lexicon};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Stats {
    words: Count,
    kanji: Count,
}

#[derive(Default, Serialize)]
pub struct Count {
    count: u32,
    unique_count: u32,
    unknown_count: u32,
    n1_n5_count: (u32, u32, u32, u32, u32),
}

pub struct WordAnalyzer {}

impl WordAnalyzer {
    pub fn new(_config: &Config) -> Self {
        Self {}
    }

    pub fn analyze(&self, lexicon: &Lexicon) -> Stats {
        Stats {
            words: Count {
                count: lexicon.words.values().map(|w| w.count).sum(),
                unique_count: lexicon.words.len() as u32,
                unknown_count: lexicon.words.values().filter(|w| !w.learned).count() as u32,
                n1_n5_count: (
                    lexicon
                        .words
                        .values()
                        .filter(|w| w.tags.contains("jlpt=N1"))
                        .count() as u32,
                    lexicon
                        .words
                        .values()
                        .filter(|w| w.tags.contains("jlpt=N2"))
                        .count() as u32,
                    lexicon
                        .words
                        .values()
                        .filter(|w| w.tags.contains("jlpt=N3"))
                        .count() as u32,
                    lexicon
                        .words
                        .values()
                        .filter(|w| w.tags.contains("jlpt=N4"))
                        .count() as u32,
                    lexicon
                        .words
                        .values()
                        .filter(|w| w.tags.contains("jlpt=N5"))
                        .count() as u32,
                ),
            },
            kanji: Count {
                count: lexicon.kanji.values().map(|w| w.count).sum(),
                unique_count: lexicon.kanji.len() as u32,
                unknown_count: lexicon
                    .kanji
                    .values()
                    .map(|k| if k.learned { 0 } else { 1 })
                    .sum(),
                ..Default::default()
            },
        }
    }
}
