mod config;
mod deck;
mod kanji;
mod lexicon;
mod word;

pub use config::Config;
pub use deck::Deck;
pub use kanji::Kanji;
pub use lexicon::Lexicon;
pub use word::{Word, WordCategory};

pub trait DictionaryItem {
    fn count(&self) -> u32;
    fn learned(&self) -> bool;
    fn tags(&self) -> impl Iterator<Item = &String>;
    fn tag(&mut self, tag: String);
}
