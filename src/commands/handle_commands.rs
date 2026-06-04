use crate::database::Database;

use super::{get_value, set_value};

pub fn handle_command(input: &str, db: &mut Database) -> String {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() >= 3 && parts[0].eq_ignore_ascii_case("SET") {
        let value = parts[2..].join(" ");
        return set_value::handle(db, parts[1], &value);
    }

    if parts.len() == 2 && parts[0].eq_ignore_ascii_case("GET") {
        return get_value::handle(db, parts[1]);
    }

    "ERR\n".to_string()
}
