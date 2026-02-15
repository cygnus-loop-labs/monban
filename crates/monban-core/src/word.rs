#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Word {
    pub word: String,
    pub cat: String,
    pub subcat: String,
    pub valid: bool,
    pub count: u32,
    pub learned: bool,
}

impl Word {
    pub fn new(word: String, cat: String, subcat: String, valid: bool) -> Self {
        Self {
            word,
            cat,
            subcat,
            valid,
            count: 1,
            learned: false,
        }
    }
}
