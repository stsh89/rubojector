use crate::cli::Cli;

pub struct Application {}

impl Application {
    pub fn run() {
        let article = newspaper::static_articles::about();

        Cli::new().render(article);
    }
}
