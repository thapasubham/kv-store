use std::sync::{Arc, RwLock};

use crate::database::Database;

use super::{CommandOutcome, exit, get_value, set_value};

pub fn handle_command(input: &str, db: &Arc<RwLock<Database>>) -> CommandOutcome {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() >= 3 && parts[0].eq_ignore_ascii_case("SET") {
        let value = parts[2..].join(" ");

        let result = {
            let mut db = db.write().unwrap();
            set_value::handle(&mut db, parts[1], &value)
        };

        return CommandOutcome::Respond(result);
    }

    if parts.len() == 2 && parts[0].eq_ignore_ascii_case("GET") {
        let result = {
            let db = db.read().unwrap();
            get_value::handle(&db, parts[1])
        };

        return CommandOutcome::Respond(result);
    }

    if parts.len() == 1 && parts[0].eq_ignore_ascii_case("EXIT") {
        return exit::handle();
    }

    CommandOutcome::Respond("ERR\n".to_string())
}
