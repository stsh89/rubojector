use crate::{writing::Writing, Article};

pub struct Designer;

impl Designer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compose(&self, writing: Writing) -> Article {
        writing.into()
    }
}
