use std::collections::{HashMap, HashSet};

use monban_core::{Config, Word};

pub struct WordFilter {
    include_filter: HashMap<String, HashSet<String>>,
    exclude_chars: Vec<String>,
}

impl WordFilter {
    pub fn new(config: &Config) -> Self {
        Self {
            include_filter: config.parser.filtering.include.clone(),
            exclude_chars: config.parser.filtering.exclude_chars.clone(),
        }
    }

    pub fn filter(&self, word: &Word) -> bool {
        if !word.valid {
            return false;
        }

        if self.exclude_chars.contains(&word.word) {
            return false;
        }

        if word.word.chars().all(|c| !c.is_alphabetic()) {
            tracing::debug!(target: "parser", "Filtering non alpha words: {}", &word.word);
            return false;
        }

        if let Some(cat) = self.include_filter.get(&word.cat) {
            cat.contains(&word.subcat)
        } else {
            tracing::debug!(target: "parser", "Filtering word: {}: {}, {}", &word.word, &word.cat, &word.subcat);
            false
        }
    }
}
