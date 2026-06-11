use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
}
