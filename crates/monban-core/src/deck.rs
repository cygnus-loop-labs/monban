use std::collections::HashSet;

use crate::Word;

pub struct Deck {
    pub words: HashSet<String>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    pub fn check(&self, word: &mut Word) {
        if self.words.contains(&word.word) {
            word.learned = true;
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
