use monban_core::{Config, Word, WordCategory};

pub struct WordFilter {
    exclude_chars: Vec<String>,
}

impl WordFilter {
    pub fn new(config: &Config) -> Self {
        Self {
            exclude_chars: config.parser.filtering.exclude_chars.clone(),
        }
    }

    pub fn filter(&self, word: &Word) -> bool {
        if self.exclude_chars.contains(&word.word) {
            return false;
        }

        if word.cat == WordCategory::Unknown {
            return false;
        }

        if word.word.chars().all(|c| !c.is_alphabetic()) {
            tracing::debug!(target: "parser", "Filtering non alpha words: {}", &word.word);
            return false;
        }

        true
    }
}
