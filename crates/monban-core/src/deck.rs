use std::collections::HashMap;

use crate::Word;

pub struct DeckEntry {
    level: u32,
}

pub struct Deck {
    name: String,
    words: HashMap<String, DeckEntry>,
    kanji: HashMap<String, DeckEntry>,
}

impl Deck {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: String, level: u32) {
        self.words.insert(word.clone(), DeckEntry { level });
    }

    pub fn add_kanji(&mut self, kanji: String, level: u32) {
        self.kanji.insert(kanji.clone(), DeckEntry { level });
    }

    pub fn check(&self, word: &mut Word) {
        if let Some(e) = self.words.get(&word.word) {
            word.learned = true;
            word.tags.push(self.name.clone());
            if word.level == 0 || word.level > e.level {
                word.level = e.level;
            }
        }
    }
}
