use crate::text::Text;

pub struct Writing {
    parts: Vec<Part>,
}

pub enum Part {
    Description(Text),
    Title(Text),
}

impl Writing {
    pub(crate) fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }

    pub fn parts(self) -> Vec<Part> {
        self.parts
    }
}
