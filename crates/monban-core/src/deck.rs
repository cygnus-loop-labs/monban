use std::collections::HashMap;

use crate::Word;

pub struct DeckEntry {
    learned: bool,
    tags: Vec<String>,
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

    pub fn add_word(&mut self, word: String, learned: bool, tags: Vec<String>) {
        self.words.insert(word.clone(), DeckEntry { learned, tags });
    }

    pub fn add_kanji(&mut self, kanji: String, learned: bool, tags: Vec<String>) {
        self.kanji
            .insert(kanji.clone(), DeckEntry { learned, tags });
    }

    pub fn check(&self, word: &mut Word) {
        if let Some(e) = self.words.get(&word.word) {
            word.learned |= e.learned;
            word.tags.push(self.name.clone());
            word.tags.extend_from_slice(&e.tags);
        }
    }
}
