use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use kanaria::string::UCSStr;
use serde::Deserialize;
use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

use crate::{
    parsing::{
        ParseError,
        dict::{Dict, DictWord},
    },
    util::load_data_file,
};

#[derive(Deserialize)]
pub struct JMWord {
    pub id: String,
    pub kanji: Vec<JMKanji>,
    pub kana: Vec<JMKana>,
    pub sense: Vec<JMSense>,
}

#[derive(Deserialize)]
pub struct JMKanji {
    pub common: bool,
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct JMKana {
    pub common: bool,
    pub text: String,
    pub tags: Vec<String>,
    #[serde(rename = "appliesToKanji")]
    pub applies_to_kanji: Vec<String>,
}

#[derive(Deserialize)]
pub struct JMSense {
    #[serde(rename = "partOfSpeech")]
    pub pos: Vec<String>,
    #[serde(rename = "appliesToKanji")]
    pub applies_to_kanji: Vec<String>,
    #[serde(rename = "appliesToKana")]
    pub applies_to_kana: Vec<String>,
    pub related: Vec<Vec<Value>>,
    pub antonym: Vec<Vec<Value>>,
    pub field: Vec<String>,
    pub dialect: Vec<String>,
    pub misc: Vec<String>,
    pub info: Vec<String>,
    #[serde(rename = "languageSource")]
    pub language_source: Vec<JMLanguageSource>,
    pub gloss: Vec<JMGloss>,
}

#[derive(Deserialize)]
pub struct JMLanguageSource {
    pub lang: String,
    pub full: bool,
    pub wasei: bool,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct JMGloss {
    pub lang: String,
    pub gender: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub text: String,
}

#[derive(Deserialize)]
pub struct JMDictLoader {
    pub version: String,
    pub languages: Vec<String>,
    #[serde(rename = "commonOnly")]
    pub common_only: bool,
    #[serde(rename = "dictDate")]
    pub dict_date: String,
    #[serde(rename = "dictRevisions")]
    pub dict_revisions: Vec<String>,
    pub tags: HashMap<String, String>,
    pub words: Vec<JMWord>,
}

impl JMDictLoader {
    pub fn new(word_file: impl AsRef<Path>) -> Result<Self, ParseError> {
        let content = load_data_file(word_file)?;

        Ok(serde_json::from_str(&content).unwrap())
    }

    fn get_base_forms(dict_word: &JMWord, kana: &JMKana) -> Vec<String> {
        let mut result = vec![];

        if kana.applies_to_kanji.is_empty() {
            // kana only word
            result.push(kana.text.clone());
        } else if kana.applies_to_kanji.iter().any(|s| s == "*") {
            if dict_word.kanji.is_empty() {
                // kana only word
                result.push(kana.text.clone());
            } else {
                // reading applies to all kanji
                for kanji in &dict_word.kanji {
                    if kanji.tags.iter().any(|s| s == "sK") {
                        continue;
                    }
                    result.push(kanji.text.clone());
                }
            }
        } else {
            // reading applies to subset of kanji
            for kanji in &kana.applies_to_kanji {
                result.push(kanji.to_string());
            }
        }

        result
    }

    fn get_senses<'a>(dict_word: &'a JMWord, base: &'a str, kana: &'a str) -> Vec<&'a JMSense> {
        let mut senses = vec![];

        for sense in &dict_word.sense {
            if !sense.applies_to_kana.iter().any(|s| s == "*" || s == kana) {
                continue;
            }
            if !sense.applies_to_kanji.iter().any(|s| s == "*" || s == base) {
                continue;
            }

            senses.push(sense);
        }

        senses
    }

    fn get_pos(dict_word: &JMWord, base: &str, kana: &str) -> HashSet<String> {
        let mut pos = HashSet::new();

        let senses = Self::get_senses(dict_word, base, kana);

        for sense in senses {
            for s_pos in &sense.pos {
                pos.insert(s_pos.to_owned());
            }
        }

        pos
    }

    pub fn parse_dict_entry(dict_word: &JMWord) -> Vec<DictWord> {
        let mut result = vec![];

        for kana in &dict_word.kana {
            if kana.tags.iter().any(|s| s == "sk") {
                continue;
            }

            let reading = Self::normalize_reading(&kana.text);

            let base_forms = Self::get_base_forms(dict_word, kana);

            for base_form in base_forms {
                let pos = Self::get_pos(dict_word, &base_form, &kana.text);

                result.push(Dict::new_word(
                    dict_word.id.clone(),
                    base_form,
                    reading.clone(),
                    pos,
                ));
            }
        }

        result
    }

    pub fn load(
        word_file: impl AsRef<Path>,
        kanji_file: impl AsRef<Path>,
    ) -> Result<Dict, ParseError> {
        let jmdict = Self::new(word_file)?;

        tracing::info!("Loaded {} words", jmdict.words.len());

        let mut dict = Dict::new();

        for dict_word in jmdict.words {
            for entry in Self::parse_dict_entry(&dict_word) {
                dict.add_word(entry);
            }
        }

        for kanji in Self::load_kanji(kanji_file)? {
            dict.add_kanji(kanji);
        }

        Ok(dict)
    }

    fn load_kanji(file: impl AsRef<Path>) -> Result<Vec<char>, ParseError> {
        let mut result = vec![];

        let content = load_data_file(file)?;

        let kanji_s: Vec<String> = serde_json::from_str(&content).unwrap();
        let kanji: Vec<char> = kanji_s.iter().filter_map(|k| k.chars().next()).collect();

        for k in kanji {
            result.push(k);
        }

        Ok(result)
    }

    fn normalize_reading(reading: &str) -> String {
        let reading = reading.nfkc().collect::<String>();

        UCSStr::from_str(&reading).katakana().to_string()
    }
}
