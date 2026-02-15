pub struct Kanji {
    pub kanji: char,
    pub count: u32,
}

impl Kanji {
    pub fn new(kanji: char) -> Self {
        Self { kanji, count: 1 }
    }
}
