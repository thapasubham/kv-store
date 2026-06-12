use std::time::Duration;

use crate::database::Database;

pub fn handle(db: &mut Database, parts: &[&str]) -> String {
    if parts.len() < 2 {
        return "ERR\n".to_string();
    }

    let key = parts[0];

    let mut value_parts = Vec::new();
    let mut ttl: Option<Duration> = None;

    let mut i = 1;

    while i < parts.len() {
        if parts[i].eq_ignore_ascii_case("EX") && i + 1 < parts.len() {
            if let Ok(sec) = parts[i + 1].parse::<u64>() {
                ttl = Some(Duration::from_secs(sec));
            }
            break;
        } else {
            value_parts.push(parts[i]);
        }
        i += 1;
    }

    let value = value_parts.join(" ");

    db.set(key, &value, ttl);

    "OK\n".to_string()
}
