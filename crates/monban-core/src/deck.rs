use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::{Kanji, Word, WordCategory};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeckEntry {
    pub word: String,
    pub reading: String,
    pub cat: WordCategory,
    pub learned: bool,
    pub tags: HashSet<String>,
}

impl DeckEntry {
    pub fn tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
}

pub struct Deck {
    words: HashMap<String, DeckEntry>,
    kanji: HashMap<String, Kanji>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: String, reading: String, cat: WordCategory) -> &mut DeckEntry {
        self.words.entry(word.clone()).or_insert(DeckEntry {
            word,
            reading,
            cat,
            learned: false,
            tags: Default::default(),
        })
    }

    pub fn add_kanji(&mut self, kanji: Kanji) {
        self.kanji.insert(kanji.kanji.to_string(), kanji);
    }

    pub fn check(&self, word: &mut Word) {
        if let Some(e) = self.words.get(&word.word) {
            word.learned |= e.learned;
            word.tags.extend(e.tags.iter().cloned());
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
