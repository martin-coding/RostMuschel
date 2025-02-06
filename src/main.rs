mod builtins;
mod call;

use crate::builtins::*;
use crate::call::Call;

use std::io::{self, Write};

// The character that is printed on every new line.
const SHELL_PROMPT: char = '>';

fn main() {
    loop {
        // Prints the shell prompt and a whitespace.
        print!("{} ", SHELL_PROMPT);
        flush_stdout();

        // Get user input.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let call = Call::from_input(&input);

        // Handle functionality and output.
        handle_command(&call);
        flush_stdout();
    }
}

/// Flushes the output buffer of stdout.
fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Handles command logic and output.
fn handle_command(call: &Call) {
    match call.command {
        "exit" => exit::call(call),
        "echo" => echo::call(call),
        "pwd" => pwd::call(call),
        _ => unknown::call(call),
    }
}
