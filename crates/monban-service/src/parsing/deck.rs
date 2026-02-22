mod jlpt;
mod plain;
mod wk;

use std::path::Path;

use monban_core::{Config, Deck};

pub use jlpt::JLPTDeckLoader;
pub use plain::PlainDeckLoader;
pub use wk::WKDeckLoader;

pub trait DeckLoader {
    fn load(name: String, file: impl AsRef<Path>, config: &Config) -> Deck;
}
