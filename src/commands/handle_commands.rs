use std::sync::{Arc, RwLock};

use crate::{commands::command_router, database::Database};

use super::CommandOutcome;

pub fn handle_command(input: &str, db: &Arc<RwLock<Database>>) -> CommandOutcome {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let Some(cmd_word) = parts.first() else {
        return CommandOutcome::Respond("ERR unknown command".into());
    };

    let cmd = cmd_word.to_ascii_uppercase();
    let Some(handler) = command_router::router().get(cmd.as_str()) else {
        return CommandOutcome::Respond("ERR unknown command".into());
    };

    handler(&parts, db)
}
