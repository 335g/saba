use alloc::string::{String, ToString};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute {
            name: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl Attribute {
    pub fn push_name(&mut self, c: char) {
        self.name.push(c);
    }

    pub fn push_value(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}
