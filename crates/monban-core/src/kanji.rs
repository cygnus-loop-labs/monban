use std::collections::HashSet;

use serde::Serialize;

use crate::DictionaryItem;

#[derive(Serialize)]
pub struct Kanji {
    pub kanji: char,
    pub count: u32,
    pub learned: bool,
    pub tags: HashSet<String>,
}

impl Kanji {
    pub fn new(kanji: char) -> Self {
        Self {
            kanji,
            count: 1,
            learned: false,
            tags: HashSet::new(),
        }
    }
}

impl DictionaryItem for Kanji {
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
