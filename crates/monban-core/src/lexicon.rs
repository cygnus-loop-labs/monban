use indexmap::{IndexMap, map::Entry};
use serde::Serialize;

use crate::{Kanji, Word};

#[derive(Serialize)]
pub struct Lexicon {
    pub words: IndexMap<String, Word>,
    pub kanji: IndexMap<char, Kanji>,
}

impl Lexicon {
    pub fn new() -> Self {
        Self {
            words: IndexMap::new(),
            kanji: IndexMap::new(),
        }
    }

    pub fn add_kanji(&mut self, kanji: char) {
        match self.kanji.entry(kanji) {
            Entry::Occupied(mut e) => {
                e.get_mut().count += 1;
            }
            Entry::Vacant(e) => {
                e.insert(Kanji::new(kanji));
            }
        }
    }

    pub fn add_word(&mut self, word: Word) {
        match self.words.entry(word.word.clone()) {
            Entry::Occupied(mut e) => {
                tracing::debug!(target = "core", "Duplicate: {}", &word.word);
                e.get_mut().count += 1;
            }
            Entry::Vacant(e) => {
                e.insert(word);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Word> {
        self.words.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Word> {
        self.words.values_mut()
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}
