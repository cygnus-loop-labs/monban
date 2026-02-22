use std::{fs, path::Path};

pub struct PlainTextLoader;

impl PlainTextLoader {
    pub fn load(file: impl AsRef<Path>) -> Vec<String> {
        vec![fs::read_to_string(file).unwrap()]
    }
}
