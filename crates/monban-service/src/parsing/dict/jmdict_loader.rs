use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use serde::Deserialize;
use serde_json::Value;

use crate::{
    parsing::{
        ParseError,
        dict::{Dict, DictKanji, DictWord},
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
    pub fn load(
        word_file: impl AsRef<Path>,
        kanji_file: impl AsRef<Path>,
    ) -> Result<Dict, ParseError> {
        let content = load_data_file(word_file)?;

        let jmdict: JMDictLoader = serde_json::from_str(&content).unwrap();

        tracing::info!("Loaded {} words", jmdict.words.len());

        let mut dict = Dict::new();

        for dict_word in jmdict.words {
            let word = if dict_word.kanji.is_empty() {
                dict_word.kana[0].text.clone()
            } else {
                dict_word.kanji[0].text.clone()
            };

            let mut pos = HashSet::new();

            for sense in &dict_word.sense {
                for s_pos in &sense.pos {
                    pos.insert(s_pos.to_owned());
                }
            }
            dict.words.insert(
                word.clone(),
                DictWord {
                    word,
                    pos: pos.into_iter().collect(),
                },
            );
        }

        for kanji in Self::load_kanji(kanji_file)? {
            dict.kanji.insert(kanji, DictKanji { kanji });
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
}
