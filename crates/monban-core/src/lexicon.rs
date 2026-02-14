use std::collections::{HashSet, hash_set};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Word {
    pub word: String,
    pub cat: String,
    pub subcat: String,
}

pub struct Lexicon {
    pub words: HashSet<Word>,
}

impl Lexicon {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    pub fn add(&mut self, word: Word) {
        self.words.insert(word);
    }

    pub fn add_all(&mut self, words: Vec<Word>) {
        for word in words {
            self.add(word);
        }
    }
}

impl IntoIterator for Lexicon {
    type Item = Word;
    type IntoIter = hash_set::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}
