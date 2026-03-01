mod jlpt;
mod plain;
mod wk;

use std::path::Path;

use monban_core::{Config, Deck};

pub use jlpt::JLPTDeckLoader;
pub use plain::PlainDeckLoader;
pub use wk::WKDeckLoader;

use crate::parsing::ParseError;

pub trait DeckLoader {
    fn load(name: String, file: impl AsRef<Path>, config: &Config) -> Result<Deck, ParseError>;
}
