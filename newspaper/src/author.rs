use crate::writing::{Part as WritingPart, Writing};

pub struct Author;

pub enum Draft {
    Title(String),
    Description(String),
}

impl From<Draft> for WritingPart {
    fn from(draft: Draft) -> Self {
        match draft {
            Draft::Title(title) => WritingPart::Title(title.into()),
            Draft::Description(description) => WritingPart::Description(description.into()),
        }
    }
}

impl Author {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, drafts: Vec<Draft>) -> Writing {
        let content = drafts.into_iter().map(Into::into).collect();

        Writing::new(content)
    }
}
