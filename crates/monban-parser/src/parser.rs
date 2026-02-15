use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, token::Token,
    tokenizer::Tokenizer,
};

use monban_core::{Config, Lexicon, Word};

use crate::{
    dict::JMDict,
    input::{EpubTextLoader, PlainTextLoader},
};

const DETAILS_CATEGORY: usize = 0;
const DETAILS_SUBCATEGORY1: usize = 1;
const DETAILS_BASE: usize = 6;

pub struct Parser {
    tokenizer: Tokenizer,
    dict: JMDict,
    include_filter: HashMap<String, HashSet<String>>,
    exclude_chars: Vec<String>,
}

impl Parser {
    pub fn new(config: &Config) -> Self {
        let ipadic = load_dictionary(&config.parser.dictionary).unwrap();
        let mut dict = JMDict::new();
        dict.load(&config.dictionary.words, &config.dictionary.kanji);

        Self {
            tokenizer: Tokenizer::new(Segmenter::new(Mode::Normal, ipadic, None)),
            dict,
            include_filter: config.parser.filtering.include.clone(),
            exclude_chars: config.parser.filtering.exclude_chars.clone(),
        }
    }

    pub fn load_text(&self, file: impl AsRef<Path>) -> Lexicon {
        self.parse_content(PlainTextLoader::load(file))
    }

    pub fn load_epub(&self, file: impl AsRef<Path>) -> Lexicon {
        self.parse_content(EpubTextLoader::load(file))
    }

    fn parse_content(&self, content: Vec<String>) -> Lexicon {
        let mut tokens: Vec<Token> = vec![];

        (0..content.len()).for_each(|i| {
            tokens.append(&mut self.tokenizer.tokenize(&content[i]).unwrap());
        });

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
