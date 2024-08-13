use crate::page::Page;

pub struct PageFormatter {
    page: Page,
}

pub struct FormatOptions {
    format: Format,
}

pub enum Format {
    Json,
}

impl PageFormatter {
    pub fn new(page: Page) -> Self {
        Self { page }
    }

    pub fn format(&self, options: FormatOptions) -> String {
        let FormatOptions { format } = options;

        match format {
            Format::Json => page_to_json(&self.page),
        }
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            format: Format::Json,
        }
    }
}

fn page_to_json(page: &Page) -> String {
    match serde_json::to_string(page) {
        Ok(json_string) => json_string,
        Err(_) => todo!(),
    }
}
