use std::path::Path;

use thiserror::Error;

use monban_core::{Config, DeckConfig, Lexicon, Word};

use crate::{
    integration::anki::ConnectError,
    parsing::{
        DeckLoader as _, InputType, JLPTDeckLoader, ParseError, Parser, PlainDeckLoader,
        WKDeckLoader, deck::AnkiDeckLoader,
    },
};

#[derive(Debug, Error)]
pub enum AnalyzeError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Connect error: {0}")]
    ConnectError(#[from] ConnectError),
}

pub fn cmd_get_blacklist(config: &Config) -> Result<Vec<Word>, ParseError> {
    let parser = Parser::new(config)?;

    let blacklist = parser.load_blacklist(&config.parser.blacklist)?;

    Ok(blacklist.into_values().collect())
}

pub async fn cmd_analyze<F>(
    config: &Config,
    input: impl AsRef<Path>,
    on_progress: F,
) -> Result<Lexicon, AnalyzeError>
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
    let n_decks = deck_list.len();

    let deck_progress = 30 / n_decks;

    let mut decks = vec![];

    for deck in deck_list {
        progress += deck_progress as u32;
        on_progress(progress);
        let deck = match deck {
            DeckConfig::Anki {
                name,
                word,
                reading,
                meaning,
            } => AnkiDeckLoader::load(name.to_string(), config, word, reading, meaning).await?,
            DeckConfig::File { name, path } => {
                PlainDeckLoader::load(name.to_string(), path, config)?
            }
            DeckConfig::Jlpt { name, path } => {
                JLPTDeckLoader::load(name.to_string(), path, config)?
            }
            DeckConfig::Wk { name, path } => WKDeckLoader::load(name.to_string(), path, config)?,
        };

        decks.push(deck);
    }

    progress = 60;
    on_progress(progress);

    for deck in decks.iter_mut() {
        tracing::info!("Checking {}", &deck.name);
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
