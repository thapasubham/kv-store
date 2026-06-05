mod exit;
mod get_value;
mod handle_commands;
mod set_value;

pub enum CommandOutcome {
    Respond(String),
    Shutdown,
}

pub use handle_commands::handle_command;
