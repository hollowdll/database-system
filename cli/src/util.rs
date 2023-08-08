// Utility functions

use std::io::{self, Write};
use crate::{
    engine_core::logging::error::LogError,
    NO_CONNECTED_DB,
};

/// Prints a message telling there is no connected database.
pub fn db_not_connected() {
    println!("{}", NO_CONNECTED_DB);
}

/// Prints an error message telling event logging failed.
pub fn event_log_failed(err: Option<LogError>) {
    if let Some(err) = err {
        eprintln!("Error: Failed to log event: {}", err);
    }
}

/// Prints an error message telling error logging failed.
pub fn error_log_failed(err: Option<LogError>) {
    if let Some(err) = err {
        eprintln!("Error: Failed to log error: {}", err);
    }
}

/// Asks for user input and returns it trimmed.
pub fn ask_user_input(text_to_ask: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{text_to_ask}");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}

/// Asks user to confirm an action, such as delete action.
/// 
/// Returns the input trimmed.
pub fn ask_action_confirm(text_to_ask: &str) -> io::Result<String> {
    let mut confirm = String::new();

    println!("{text_to_ask}");
    print!("'Y' to confirm: ");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut confirm) {
        eprintln!("Failed to read line: {e}");
        return Err(e);
    }
    let confirm = confirm.trim().to_string();

    Ok(confirm)
}
