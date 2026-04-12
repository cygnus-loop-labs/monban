pub mod deck;
pub mod dict;
pub mod input;

pub use self::deck::{DeckLoader, JLPTDeckLoader, PlainDeckLoader, WKDeckLoader};

use std::{collections::HashMap, path::Path};

use indexmap::IndexMap;
use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, token::Token,
    tokenizer::Tokenizer,
};

use monban_core::{Config, Lexicon, Word, WordCategory};
use thiserror::Error;

use crate::{
    analysis::filtering::WordFilter,
    parsing::dict::{Dict, JMDictLoader},
    util::load_data_file,
};

use self::input::{EpubTextLoader, PlainTextLoader};

#[allow(unused)]
enum TokenDetails {
    Category = 0,
    Subcategory1 = 1,
    Subcategory2 = 2,
    Subcategory3 = 3,
    ConjugationType = 4,
    ConjugationForm = 5,
    Base = 6,
    Reading = 7,
    Pronunciation = 8,
}

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
    dict: Dict,
    filter: WordFilter,
    mapper: HashMap<WordCategory, Vec<(String, String)>>,
}

impl Parser {
    pub fn new(config: &Config) -> Result<Self, ParseError> {
        let ipadic = load_dictionary(&config.parser.dictionary).unwrap();
        let dict = JMDictLoader::load(&config.dictionary.words, &config.dictionary.kanji)?;

        Ok(Self {
            tokenizer: Tokenizer::new(Segmenter::new(Mode::Normal, ipadic, None)),
            dict,
            filter: WordFilter::new(config),
            mapper: config.parser.mapping.clone(),
        })
    }

    pub fn load_blacklist(
        &self,
        file: impl AsRef<Path>,
    ) -> Result<IndexMap<String, Word>, ParseError> {
        Ok(load_data_file(file)?
            .lines()
            .map(|l| {
                (
                    l.to_string(),
                    Word::new(l.to_string(), WordCategory::Unknown),
                )
            })
            .collect())
    }

    pub fn load_text(
        &self,
        file: impl AsRef<Path>,
        blacklist: &IndexMap<String, Word>,
    ) -> Result<Lexicon, ParseError> {
        self.parse_content(PlainTextLoader::load(file)?, blacklist)
    }

    pub fn load_epub(
        &self,
        file: impl AsRef<Path>,
        blacklist: &IndexMap<String, Word>,
    ) -> Result<Lexicon, ParseError> {
        self.parse_content(EpubTextLoader::load(file)?, blacklist)
    }

    fn parse_token(&self, token: &mut Token) -> Word {
        let surface = token.surface.trim().to_string();
        let details = token.details();

        let word = if details[TokenDetails::Base as usize] == "*" {
            surface
        } else {
            details[TokenDetails::Base as usize].to_string()
        };

        let cat = details[TokenDetails::Category as usize].to_string();
        let subcat = details[TokenDetails::Subcategory1 as usize].to_string();

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
    }

    fn parse_content(
        &self,
        content: Vec<String>,
        blacklist: &IndexMap<String, Word>,
    ) -> Result<Lexicon, ParseError> {
        let mut tokens: Vec<Token> = vec![];

        (0..content.len()).for_each(|i| {
            tokens.append(&mut self.tokenizer.tokenize(&content[i]).unwrap());
        });

        let mut no_match = 0;
        let mut filtered = 0;
        let mut blacklisted = 0;

        let words = tokens
            .iter_mut()
            .map(|token| {
                let mut word = self.parse_token(token);

                if !self.filter.filter(&word) {
                    if blacklist.contains_key(&word.word) {
                        blacklisted += 1;
                        word.filter = true;
                    } else if !self.dict.contains_word(&word) {
                        tracing::debug!("No match {} [{:?}]", &word.word, &token.details());
                        no_match += 1;
                        word.filter = true;
                    }
                }

                word
            })
            .filter(|word| {
                if self.filter.filter(word) {
                    filtered += 1;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<Word>>();

        let mut lex = Lexicon::new();
        lex.tokens = tokens.len();
        lex.skipped = no_match;
        lex.blacklisted = blacklisted;
        lex.filtered = filtered;

        for word in words {
            for c in word.word.chars() {
                if self.dict.contains_kanji(c) {
                    lex.add_kanji(c);
                }
            }
            lex.add_word(word);
        }

        Ok(lex)
    }
}
