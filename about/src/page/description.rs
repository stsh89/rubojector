use serde::{Serialize, Serializer};

pub struct Description {
    value: String,
}

impl Serialize for Description {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl Description {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
