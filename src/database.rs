use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
#[derive(Debug)]
pub struct Entry {
    value: String,
    expires_at: Option<Instant>,
}
pub struct Database {
    data: HashMap<String, Entry>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str, ttl: Option<Duration>) {
        let expires_at = ttl.map(|ttl| Instant::now() + ttl);

        self.data.insert(
            key.to_string(),
            Entry {
                value: value.to_string(),
                expires_at,
            },
        );
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let expired = match self.data.get(key) {
            Some(entry) => entry.expires_at.is_some_and(|exp| Instant::now() >= exp),
            None => return None,
        };

        if expired {
            self.data.remove(key);
            return None;
        }

        self.data.get(key).map(|e| e.value.clone())
    }

    pub fn delete(&mut self, key: &str) -> bool {
        let expired = match self.data.get(key) {
            Some(entry) => entry.expires_at.is_some_and(|exp| Instant::now() >= exp),
            None => return false,
        };

        if expired {
            self.data.remove(key);
            return false;
        }

        self.data.remove(key).is_some()
    }
}
