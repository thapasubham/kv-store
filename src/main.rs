mod commands;
mod database;
mod server;

use database::Database;

fn main() {
    let addr = "127.0.0.1:5000";
    let mut db = Database::new();
    server::run(addr, &mut db);
}
