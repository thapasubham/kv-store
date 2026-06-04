use crate::database::Database;

pub fn handle(db: &Database, key: &str) -> String {
    match db.get(key) {
        Some(value) => value.clone(),
        None => "NOT FOUND\n".to_string(),
    }
}
