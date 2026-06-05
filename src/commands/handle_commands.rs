use crate::database::Database;

use super::{exit, get_value, set_value, CommandOutcome};

pub fn handle_command(input: &str, db: &mut Database) -> CommandOutcome {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() >= 3 && parts[0].eq_ignore_ascii_case("SET") {
        let value = parts[2..].join(" ");
        let result = set_value::handle(db, parts[1], &value);
        return CommandOutcome::Respond(result);
    }

    if parts.len() == 2 && parts[0].eq_ignore_ascii_case("GET") {
        let result = get_value::handle(db, parts[1]);
        return CommandOutcome::Respond(result);
    }

    if parts.len() == 1 && parts[0].eq_ignore_ascii_case("EXIT") {
        return exit::handle();
    }

    CommandOutcome::Respond("ERR\n".to_string())
}
