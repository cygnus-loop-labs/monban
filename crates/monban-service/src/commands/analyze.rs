use std::path::Path;

use crate::parsing::{
    DeckLoader as _, InputType, JLPTDeckLoader, ParseError, Parser, PlainDeckLoader, WKDeckLoader,
};
use monban_core::{Config, Deck, Lexicon};

pub fn cmd_analyze<F>(
    config: &Config,
    input: impl AsRef<Path>,
    ty: InputType,
    on_progress: F,
) -> Result<Lexicon, ParseError>
where
    F: Fn(u32),
{
    on_progress(0);

    let parser = Parser::new(config)?;

    let blacklist = parser.load_blacklist(&config.parser.blacklist)?;

    on_progress(10);

    let mut lexicon = match ty {
        InputType::Txt => parser.load_text(input, &blacklist),
        InputType::Epub => parser.load_epub(input, &blacklist),
    }?;

    on_progress(20);

    let decks = &mut config
        .decks
        .iter()
        .map(|(name, params)| match params.ty.as_str() {
            "plain" => PlainDeckLoader::load(name.to_string(), &params.file, config),
            "wk" => WKDeckLoader::load(name.to_string(), &params.file, config),
            "jlpt" => JLPTDeckLoader::load(name.to_string(), &params.file, config),
            _ => unimplemented!(),
        })
        .collect::<Result<Vec<Deck>, ParseError>>()?;

    on_progress(50);

    for word in lexicon.iter_mut() {
        for deck in decks.iter_mut() {
            deck.check(word);
        }
    }

    on_progress(100);

    Ok(lexicon)
}
