use std::sync::{Arc, RwLock};

use crate::database::Database;

use super::CommandOutcome;

pub fn handle(db: &mut Database, key: &str) -> String {
    if db.delete(key) {
        "OK\n".to_string()
    } else {
        "NOT FOUND\n".to_string()
    }
}

pub fn handle_delete(parts: &[&str], db: &Arc<RwLock<Database>>) -> CommandOutcome {
    if parts.len() != 2 {
        return CommandOutcome::Respond("ERR\n".into());
    }

    let mut guard = match db.write() {
        Ok(guard) => guard,
        Err(_) => return CommandOutcome::Respond("internal error".into()),
    };

    let response = handle(&mut guard, parts[1]);
    CommandOutcome::Respond(response)
}
