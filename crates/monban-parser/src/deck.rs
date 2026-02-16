mod plain;
mod wk;

use std::path::Path;

use monban_core::Deck;

pub use plain::PlainDeckLoader;
pub use wk::WKDeckLoader;

pub trait DeckLoader {
    fn load(name: &str, file: impl AsRef<Path>) -> Deck;
}
