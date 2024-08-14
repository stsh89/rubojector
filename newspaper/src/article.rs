use crate::writing::{Part as WritingPart, Writing};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Article {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl From<Writing> for Article {
    fn from(writing: Writing) -> Self {
        let mut article = Self::default();

        for part in writing.parts() {
            match part {
                WritingPart::Description(description) => {
                    article.description = Some(description.into())
                }
                WritingPart::Title(title) => article.title = Some(title.into()),
            }
        }

        article
    }
}
