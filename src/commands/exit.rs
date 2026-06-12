use std::sync::{Arc, RwLock};

use crate::database::Database;

use super::CommandOutcome;

pub fn handle() -> CommandOutcome {
    CommandOutcome::Shutdown
}

pub fn handle_exit(_parts: &[&str], _db: &Arc<RwLock<Database>>) -> CommandOutcome {
    handle()
}
