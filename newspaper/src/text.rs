pub struct Text(String);

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.0
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Text {
    pub fn new(text: String) -> Self {
        Self(text)
    }
}
