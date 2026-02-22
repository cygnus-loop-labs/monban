use std::collections::{HashMap, HashSet};

use crate::Word;

pub struct DeckEntry {
    learned: bool,
    tags: HashSet<String>,
}

pub struct Deck {
    words: HashMap<String, DeckEntry>,
    kanji: HashMap<String, DeckEntry>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: String, learned: bool, tags: HashSet<String>) {
        self.words.insert(word.clone(), DeckEntry { learned, tags });
    }

    pub fn add_kanji(&mut self, kanji: String, learned: bool, tags: HashSet<String>) {
        self.kanji
            .insert(kanji.clone(), DeckEntry { learned, tags });
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
