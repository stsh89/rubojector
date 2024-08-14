use crate::{
    author::{Author, Draft},
    designer::Designer,
    Article,
};

pub fn about() -> Article {
    let writing = Author::new().write(vec![Draft::Description("A tool to visually represent and analyze Ruby codebases, providing interactive maps of modules, classes, methods, and relationships to improve code comprehension, navigation, and collaboration.".into())]);

    Designer::new().compose(writing)
}
