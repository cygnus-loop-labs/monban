mod jmdict_loader;

use std::collections::{HashMap, HashSet, hash_map::Entry};

use monban_core::Word;

pub use jmdict_loader::JMDictLoader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DictWord {
    pub id: String,
    pub word: String,
    pub reading: HashSet<String>,
    pub pos: HashSet<String>,
}

pub struct DictKanji {
    pub kanji: char,
}

pub struct Dict {
    words: HashMap<String, DictWord>,
    kanji: HashMap<char, DictKanji>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }

    pub fn new_word(id: String, word: String, reading: String, pos: HashSet<String>) -> DictWord {
        DictWord {
            id,
            word,
            reading: HashSet::from([reading]),
            pos,
        }
    }

    pub fn add_word(&mut self, word: DictWord) {
        let key = word.word.clone();

        match self.words.entry(key) {
            Entry::Occupied(mut e) => {
                e.get_mut().reading.extend(word.reading);
                e.get_mut().pos.extend(word.pos);
            }
            Entry::Vacant(e) => {
                e.insert(word);
            }
        }
    }

    pub fn add_kanji(&mut self, kanji: char) {
        self.kanji.insert(kanji, DictKanji { kanji });
    }

    pub fn contains_word(&self, word: &Word) -> bool {
        let key = &word.word;
        self.words.contains_key(key)
    }

    pub fn contains_kanji(&self, kanji: char) -> bool {
        self.kanji.contains_key(&kanji)
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self::new()
    }
}
