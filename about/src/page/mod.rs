mod description;

pub use description::Description;

#[derive(serde::Serialize)]
pub struct Page {
    description: description::Description,
}

pub struct PageParameters {
    pub description: description::Description,
}

impl Page {
    pub fn new(parameters: PageParameters) -> Self {
        let PageParameters { description } = parameters;

        Self { description }
    }
}
