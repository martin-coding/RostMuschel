use std::io::{self, Write};
use std::str::SplitWhitespace;

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

/// Represent a command call with arguments.
struct Call<'a> {
    command: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Call<'a> {
    /// Turns an input string into a command call.
    fn from_input(input: &'a str) -> Self {
        // Split the input into a command and arguments.
        let mut split: SplitWhitespace = input.split_whitespace();
        let command: &str = split.next().unwrap_or_default();
        let arguments: Vec<&str> = split.collect();
        Call { command, arguments }
    }
}

/// Flushes the output buffer of stdout.
fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Handles command logic and output.
fn handle_command(call: &Call) {
    match call.command {
        "exit" => {
            // Check if correct number of arguments provided.
            if call.arguments.is_empty() {
                // Always exit without error value if no argument provided.
                std::process::exit(0)
            } else if call.arguments.len() > 1 {
                println!("{}: too many arguments provided", call.command);
                return;
            }

            // Parse the argument to an integer.
            let exit_value: i32 = match call.arguments[0].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}: must provide a number", call.command);
                    return;
                }
            };

            // Now exit with provided exit value.
            std::process::exit(exit_value)
        }
        "echo" => println!("{}", call.arguments.join(" ")),
        _ => println!("{}: command not found", call.command),
    }
}
