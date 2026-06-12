mod command_router;
mod delete_value;
mod exit;
mod get_value;
mod handle_commands;
mod help;
mod set_value;

pub enum CommandOutcome {
    Respond(String),
    Shutdown,
}

pub use handle_commands::handle_command;
