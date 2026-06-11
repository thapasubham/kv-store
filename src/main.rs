mod commands;
mod database;
mod server;

use std::sync::{Arc, RwLock};

use database::Database;

fn main() {
    let addr = "127.0.0.1:5000";
    let db = Arc::new(RwLock::new(Database::new()));
    server::run(addr, db);
}
