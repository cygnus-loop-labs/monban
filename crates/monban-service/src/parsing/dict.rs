mod jmdict_loader;

use std::collections::HashMap;

pub use jmdict_loader::JMDictLoader;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DictWord {
    pub word: String,
    pub pos: Vec<String>,
}

pub struct DictKanji {
    pub kanji: char,
}

pub struct Dict {
    pub words: HashMap<String, DictWord>,
    pub kanji: HashMap<char, DictKanji>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            kanji: HashMap::new(),
        }
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self::new()
    }
}
