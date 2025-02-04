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

        // Split the input into a command and arguments.
        let mut split: SplitWhitespace = input.split_whitespace();
        let command: &str = split.next().unwrap_or_default();
        let arguments: Vec<&str> = split.collect();

        // Handle functionality and output.
        handle_command(command, arguments.as_slice());
        flush_stdout();
    }
}

/// Flushes the output buffer of stdout.
fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Handles command logic and output.
fn handle_command(command: &str, arguments: &[&str]) {
    match command {
        "exit" => {
            // Check if correct number of arguments provided.
            if arguments.is_empty() {
                // Always exit without error value if no argument provided.
                std::process::exit(0)
            } else if arguments.len() > 1 {
                println!("{}: too many arguments provided", command);
                return;
            }

            // Parse the argument to an integer.
            let exit_value: i32 = match arguments[0].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}: must provide a number", command);
                    return;
                }
            };

            // Now exit with provided exit value.
            std::process::exit(exit_value)
        }
        "echo" => println!("{}", arguments.join(" ")),
        _ => println!("{}: command not found", command),
    }
}
