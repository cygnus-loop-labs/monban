use std::collections::{HashMap, HashSet};

use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, tokenizer::Tokenizer,
};

use monban_core::{Lexicon, Word};

use crate::dict::JMDict;

const DETAILS_CATEGORY: usize = 0;
const DETAILS_SUBCATEGORY1: usize = 1;
const DETAILS_BASE: usize = 6;

pub struct Parser {
    tokenizer: Tokenizer,
    dict: JMDict,
}

impl Parser {
    pub fn new() -> Self {
        let ipadic = load_dictionary("embedded://ipadic").unwrap();
        let mut dict = JMDict::new();
        dict.load("data/jmdict.json", "data/kanji.json");

        Self {
            tokenizer: Tokenizer::new(Segmenter::new(Mode::Normal, ipadic, None)),
            dict,
        }
    }

    pub fn load_text(&self, text: &str) -> Lexicon {
        let mut tokens = self.tokenizer.tokenize(text).unwrap();

        // 0-3: category, sub cat1, sub cat 2, sub cat 3
        // 4-5: conjugation
        // 6-8: base, reading, pronunciation

        let words = tokens
            .iter_mut()
            .map(|token| {
                let surface = token.surface.trim().to_string();
                let details = token.details();

                let word = if details[DETAILS_BASE] == "*" {
                    surface
                } else {
                    details[DETAILS_BASE].to_string()
                };

                let valid = self.dict.words.contains(&word);

                Word::new(
                    word,
                    details[DETAILS_CATEGORY].to_string(),
                    details[DETAILS_SUBCATEGORY1].to_string(),
                    valid,
                )
            })
            .filter(|word| self.filter(word))
            .collect::<Vec<Word>>();

        let mut lex = Lexicon::new();
        for word in words {
            for c in word.word.chars() {
                if self.dict.kanji.contains(&c) {
                    lex.add_kanji(c);
                }
            }
            lex.add_word(word);
        }

        lex
    }

    fn filter(&self, word: &Word) -> bool {
        let cat_filters = HashMap::from([
            ("名詞", HashSet::from(["一般", "サ変接続"])),
            ("動詞", HashSet::from(["自立"])),
        ]);

        let blacklist = HashSet::from(["ー", "〜"]);

        if !word.valid {
            return false;
        }

        if blacklist.contains(&*word.word) {
            return false;
        }

        if word.word.chars().all(|c| !c.is_alphabetic()) {
            tracing::debug!(target: "parser", "Filtering non alpha words: {}", &word.word);
            return false;
        }

        if let Some(cat) = cat_filters.get(&*word.cat) {
            cat.contains(&*word.subcat)
        } else {
            tracing::debug!(target: "parser", "Filtering word: {}: {}, {}", &word.word, &word.cat, &word.subcat);
            false
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
