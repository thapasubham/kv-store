use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            db: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.db.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }
}
