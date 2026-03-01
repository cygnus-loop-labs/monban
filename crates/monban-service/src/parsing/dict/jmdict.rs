use std::{collections::HashSet, path::Path};

use crate::{parsing::ParseError, util::load_file};

pub struct JMDict {
    pub words: HashSet<String>,
    pub kanji: HashSet<char>,
}

impl JMDict {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
            kanji: HashSet::new(),
        }
    }

    pub fn load(
        &mut self,
        dict: impl AsRef<Path>,
        kanji: impl AsRef<Path>,
    ) -> Result<(), ParseError> {
        self.load_jmdict(dict)?;
        self.load_kanji(kanji)?;

        Ok(())
    }

    pub fn load_jmdict(&mut self, file: impl AsRef<Path>) -> Result<(), ParseError> {
        let content = load_file(file)?;

        let words: Vec<String> = serde_json::from_str(&content).unwrap();

        for word in words {
            self.words.insert(word);
        }

        Ok(())
    }

    pub fn load_kanji(&mut self, file: impl AsRef<Path>) -> Result<(), ParseError> {
        let content = load_file(file)?;

        let kanji_s: Vec<String> = serde_json::from_str(&content).unwrap();
        let kanji: Vec<char> = kanji_s.iter().filter_map(|k| k.chars().next()).collect();

        for k in kanji {
            self.kanji.insert(k);
        }

        Ok(())
    }
}

impl Default for JMDict {
    fn default() -> Self {
        Self::new()
    }
}
