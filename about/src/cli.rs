use newspaper::Article;

pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, article: Article) {
        match serde_json::to_string(&article) {
            Ok(json) => println!("{json}"),
            Err(_) => todo!("https://github.com/stsh89/rubojector/issues/1"),
        }
    }
}
