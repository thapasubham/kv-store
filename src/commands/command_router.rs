use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use crate::database::Database;

use super::{CommandOutcome, delete_value, exit, get_value, help, set_value};

type Handler = fn(&[&str], &Arc<RwLock<Database>>) -> CommandOutcome;

static ROUTER: OnceLock<HashMap<&'static str, Handler>> = OnceLock::new();

fn build_router() -> HashMap<&'static str, Handler> {
    let mut map = HashMap::new();
    map.insert("GET", get_value::handle_get as Handler);
    map.insert("SET", set_value::handle_set as Handler);
    map.insert("DELETE", delete_value::handle_delete as Handler);
    map.insert("EXIT", exit::handle_exit as Handler);
    map.insert("HELP", help::handle_help as Handler);
    map
}

pub fn router() -> &'static HashMap<&'static str, Handler> {
    ROUTER.get_or_init(build_router)
}
