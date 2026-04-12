use std::path::Path;

use crate::parsing::{
    DeckLoader as _, InputType, JLPTDeckLoader, ParseError, Parser, PlainDeckLoader, WKDeckLoader,
};
use monban_core::{Config, Deck, Lexicon, Word};

pub fn cmd_get_blacklist(config: &Config) -> Result<Vec<Word>, ParseError> {
    let parser = Parser::new(config)?;

    let blacklist = parser.load_blacklist(&config.parser.blacklist)?;

    Ok(blacklist.into_values().collect())
}

pub fn cmd_analyze<F>(
    config: &Config,
    input: impl AsRef<Path>,
    on_progress: F,
) -> Result<Lexicon, ParseError>
where
    F: Fn(u32),
{
    let mut progress = 0;

    on_progress(progress);

    let parser = Parser::new(config)?;

    let blacklist = parser.load_blacklist(&config.parser.blacklist)?;

    progress = 10;
    on_progress(progress);

    let ty = match input.as_ref().extension() {
        Some(ext) => {
            if ext == "epub" {
                InputType::Epub
            } else {
                InputType::Txt
            }
        }
        None => InputType::Txt,
    };
    let mut lexicon = match ty {
        InputType::Txt => parser.load_text(input, &blacklist),
        InputType::Epub => parser.load_epub(input, &blacklist),
    }?;

    progress = 20;
    on_progress(progress);

    let deck_list = &config.decks;
    let n_decks = deck_list.keys().len();

    let deck_progress = 30 / n_decks;

    let mut decks = deck_list
        .iter()
        .map(|(name, params)| {
            progress += deck_progress as u32;
            on_progress(progress);
            match params.ty.as_str() {
                "plain" => PlainDeckLoader::load(name.to_string(), &params.file, config),
                "wk" => WKDeckLoader::load(name.to_string(), &params.file, config),
                "jlpt" => JLPTDeckLoader::load(name.to_string(), &params.file, config),
                _ => unimplemented!(),
            }
        })
        .collect::<Result<Vec<Deck>, ParseError>>()?;

    progress = 60;
    on_progress(progress);

    for deck in decks.iter_mut() {
        progress += deck_progress as u32;
        on_progress(progress);
        for word in lexicon.iter_mut() {
            deck.check(word);
        }
    }

    progress = 100;
    on_progress(progress);

    Ok(lexicon)
}
