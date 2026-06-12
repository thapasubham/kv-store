use std::sync::{Arc, RwLock};

use crate::database::Database;

use super::CommandOutcome;

pub fn text() -> &'static str {
    "GET key - get value by key\n\
     SET key value [EX seconds] - set value with optional TTL\n\
     HELP - show this message\n\
     EXIT - disconnect\n"
}

pub fn handle_help(_parts: &[&str], _db: &Arc<RwLock<Database>>) -> CommandOutcome {
    CommandOutcome::Respond(text().into())
}
