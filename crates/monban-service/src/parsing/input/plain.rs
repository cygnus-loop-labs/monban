use std::path::Path;

use crate::{parsing::ParseError, util::load_file};

pub struct PlainTextLoader;

impl PlainTextLoader {
    pub fn load(file: impl AsRef<Path>) -> Result<Vec<String>, ParseError> {
        Ok(vec![load_file(file)?])
    }
}
