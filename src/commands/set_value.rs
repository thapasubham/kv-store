use crate::database::Database;

pub fn handle(db: &mut Database, key: &str, value: &str) -> String {
    db.set(key, value);
    "OK\n".to_string()
}
