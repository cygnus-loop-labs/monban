use std::collections::HashMap;

use crate::{Kanji, Word};

pub struct Deck {
    words: HashMap<String, Word>,
    kanji: HashMap<String, Kanji>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.words.insert(word.word.clone(), word);
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
