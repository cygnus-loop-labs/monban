use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::DictionaryItem;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WordCategory {
    Unknown,
    Noun,
    Verb,
    Misc,
    Adjective,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Word {
    pub word: String,
    pub cat: WordCategory,
    pub count: u32,
    pub learned: bool,
    pub tags: HashSet<String>,
}

impl Word {
    pub fn new(word: String, cat: WordCategory) -> Self {
        Self {
            word,
            cat,
            count: 1,
            learned: false,
            tags: HashSet::new(),
        }
    }
}

impl DictionaryItem for Word {
    fn count(&self) -> u32 {
        self.count
    }

    fn learned(&self) -> bool {
        self.learned
    }

    fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    fn tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
}
