use std::{collections::HashSet, path::Path};

use monban_core::{Config, Deck, Lexicon};

use crate::parsing::{DeckLoader as _, JLPTDeckLoader, Parser, PlainDeckLoader, WKDeckLoader};

#[derive(Clone)]
pub enum InputType {
    Txt,
    Epub,
}

pub fn cmd_analyze(
    config: &Config,
    input: impl AsRef<Path>,
    ty: InputType,
    blacklist: Option<impl AsRef<Path>>,
) -> Lexicon {
    let parser = Parser::new(config);

    let blacklist = if let Some(blacklist) = blacklist {
        parser.load_blacklist(blacklist)
    } else {
        HashSet::default()
    };

    let mut lexicon = match ty {
        InputType::Txt => parser.load_text(input, &blacklist),
        InputType::Epub => parser.load_epub(input, &blacklist),
    };

    let decks = &mut config
        .decks
        .iter()
        .map(|(name, params)| match params.ty.as_str() {
            "plain" => PlainDeckLoader::load(name.to_string(), &params.file, config),
            "wk" => WKDeckLoader::load(name.to_string(), &params.file, config),
            "jlpt" => JLPTDeckLoader::load(name.to_string(), &params.file, config),
            _ => unimplemented!(),
        })
        .collect::<Vec<Deck>>();

    for word in lexicon.iter_mut() {
        for deck in decks.iter_mut() {
            deck.check(word);
        }
    }

    lexicon
}
