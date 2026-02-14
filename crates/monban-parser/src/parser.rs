use std::collections::{HashMap, HashSet};

use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, tokenizer::Tokenizer,
};

use monban_core::{Lexicon, Word};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new() -> Self {
        let dict = load_dictionary("embedded://ipadic").unwrap();

        Self {
            tokenizer: Tokenizer::new(Segmenter::new(Mode::Normal, dict, None)),
        }
    }

    pub fn load_text(&self, text: &str) -> Lexicon {
        let mut tokens = self.tokenizer.tokenize(text).unwrap();

        // 0-3: category,sub cat1, sub cat 2, sub cat 3
        // 4-5: conjugation
        // 6-8: base, reading, pronunciation
        // for mut token in tokens {
        // println!("{:?}", token.details());
        // }

        let words = tokens
            .iter_mut()
            .map(|token| {
                let surface = token.surface.trim().to_string();
                let details = token.details();

                let word = if details[6] == "*" {
                    surface
                } else {
                    details[6].to_string()
                };

                Word {
                    word,
                    cat: details[0].to_string(),
                    subcat: details[1].to_string(),
                }
            })
            .filter(|word| self.filter(word))
            .collect::<Vec<Word>>();

        let mut lex = Lexicon::new();
        lex.add_all(words);

        lex
    }

    fn filter(&self, word: &Word) -> bool {
        let filters = HashMap::from([
            ("名詞", HashSet::from(["一般", "サ変接続"])),
            ("動詞", HashSet::from(["自立"])),
        ]);

        if word.word.chars().all(|c| !c.is_alphabetic()) {
            tracing::debug!(target: "parser", "Filtering non alpha words: {}", &word.word);
            return false;
        }

        if let Some(cat) = filters.get(&*word.cat) {
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
