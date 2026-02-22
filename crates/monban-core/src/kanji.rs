use serde::Serialize;

#[derive(Serialize)]
pub struct Kanji {
    pub kanji: char,
    pub count: u32,
    pub learned: bool,
}

impl Kanji {
    pub fn new(kanji: char) -> Self {
        Self {
            kanji,
            count: 1,
            learned: false,
        }
    }
}
