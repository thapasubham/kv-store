use crate::database::Database;

pub fn handle(db: &Database, key: &str) -> String {
    match db.get(key) {
        Some(value) => format!("{value}\n"),
        None => "NOT FOUND\n".to_string(),
    }
}
