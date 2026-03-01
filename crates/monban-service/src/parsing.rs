pub mod deck;
pub mod dict;
pub mod input;

pub use self::deck::{DeckLoader, JLPTDeckLoader, PlainDeckLoader, WKDeckLoader};

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, token::Token,
    tokenizer::Tokenizer,
};

use monban_core::{Config, Lexicon, Word, WordCategory};
use thiserror::Error;

use crate::{analysis::filtering::WordFilter, util::load_file};

use self::{
    dict::JMDict,
    input::{EpubTextLoader, PlainTextLoader},
};

const DETAILS_CATEGORY: usize = 0;
const DETAILS_SUBCATEGORY1: usize = 1;
const DETAILS_BASE: usize = 6;

#[derive(Debug, Clone)]
pub enum InputType {
    Txt,
    Epub,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid file type: {0:?}")]
    InvalidFileType(InputType),
    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
}

pub struct Parser {
    tokenizer: Tokenizer,
    dict: JMDict,
    filter: WordFilter,
    mapper: HashMap<WordCategory, Vec<(String, String)>>,
}

impl Parser {
    pub fn new(config: &Config) -> Result<Self, ParseError> {
        let ipadic = load_dictionary(&config.parser.dictionary).unwrap();
        let mut dict = JMDict::new();
        dict.load(&config.dictionary.words, &config.dictionary.kanji)?;

        Ok(Self {
            tokenizer: Tokenizer::new(Segmenter::new(Mode::Normal, ipadic, None)),
            dict,
            filter: WordFilter::new(config),
            mapper: config.parser.mapping.clone(),
        })
    }

    pub fn load_blacklist(&self, file: impl AsRef<Path>) -> Result<HashSet<String>, ParseError> {
        Ok(load_file(file)?.lines().map(|l| l.to_string()).collect())
    }

    pub fn load_text(
        &self,
        file: impl AsRef<Path>,
        blacklist: &HashSet<String>,
    ) -> Result<Lexicon, ParseError> {
        self.parse_content(PlainTextLoader::load(file)?, blacklist)
    }

    pub fn load_epub(
        &self,
        file: impl AsRef<Path>,
        blacklist: &HashSet<String>,
    ) -> Result<Lexicon, ParseError> {
        self.parse_content(EpubTextLoader::load(file)?, blacklist)
    }

    fn parse_content(
        &self,
        content: Vec<String>,
        blacklist: &HashSet<String>,
    ) -> Result<Lexicon, ParseError> {
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

                let cat = details[DETAILS_CATEGORY].to_string();
                let subcat = details[DETAILS_SUBCATEGORY1].to_string();

                let word_cat = {
                    let mut word_cat = WordCategory::Unknown;
                    'outer: for (c, patterns) in &self.mapper {
                        for pattern in patterns {
                            if pattern.0 == cat && (pattern.1 == subcat || pattern.1 == "*") {
                                word_cat = *c;
                                break 'outer;
                            }
                        }
                    }

                    word_cat
                };

                if word_cat == WordCategory::Unknown {
                    tracing::debug!(target: "Parser",
                        "Category no found: {}: {}/{}", word, cat, subcat);
                }

                Word::new(word, word_cat)
            })
            .filter(|word| {
                if self.dict.words.contains(&word.word) {
                    if blacklist.contains(&word.word) {
                        false
                    } else {
                        self.filter.filter(word)
                    }
                } else {
                    false
                }
            })
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

        Ok(lex)
    }
}
