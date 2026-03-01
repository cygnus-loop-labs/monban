use std::path::Path;

use crate::{
    analysis::analyzer::{Stats, WordAnalyzer},
    parsing::{
        DeckLoader as _, InputType, JLPTDeckLoader, ParseError, Parser, PlainDeckLoader,
        WKDeckLoader,
    },
};
use monban_core::{Config, Deck, Lexicon};

pub fn cmd_analyze(
    config: &Config,
    input: impl AsRef<Path>,
    ty: InputType,
) -> Result<Lexicon, ParseError> {
    let parser = Parser::new(config)?;

    let blacklist = parser.load_blacklist(&config.parser.blacklist)?;

    let mut lexicon = match ty {
        InputType::Txt => parser.load_text(input, &blacklist),
        InputType::Epub => parser.load_epub(input, &blacklist),
    }?;

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

    for word in lexicon.iter_mut() {
        for deck in decks.iter_mut() {
            deck.check(word);
        }
    }

    Ok(lexicon)
}

pub fn cmd_stats(config: &Config, lexicon: Option<&Lexicon>) -> Stats {
    let analyzer = WordAnalyzer::new(config);

    if let Some(lexicon) = lexicon {
        analyzer.analyze(lexicon)
    } else {
        Stats::default()
    }
}
