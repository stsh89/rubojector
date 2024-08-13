mod page;
mod page_formatter;

use page::{Description, Page, PageParameters};
use page_formatter::{FormatOptions, PageFormatter};

fn main() {
    let page = Page::new(PageParameters {
        description: Description::new("A tool to visually represent and analyze Ruby codebases, providing interactive maps of modules, classes, methods, and relationships to improve code comprehension, navigation, and collaboration.".to_string()),
    });

    let display_text = PageFormatter::new(page).format(FormatOptions::default());

    println!("{}", display_text);
}
