use std::io;
use std::io::Write;
use std::str::SplitWhitespace;

// The character that is printed on every new line.
const SHELL_PROMPT: char = '>';

fn main() {
    loop {
        // Prints the shell character and a whitespace.
        print!("{} ", SHELL_PROMPT);
        flush_stdout();

        // Get user input.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // Split the input into a command and parameters.
        let mut split: SplitWhitespace = input.split_whitespace();
        let command: &str = split.next().unwrap_or_default();
        let parameters: Vec<&str> = split.collect();

        // Handle functionality and output.
        handle_command(command, parameters.as_slice());
        flush_stdout();
    }
}

/// Flushes the output buffer of stdout.
fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Handles command logic and output.
fn handle_command(command: &str, parameters: &[&str]) {
    match command {
        "exit" => {
            // Check if correct number of parameters provided.
            if parameters.is_empty() {
                println!("{}: no parameter provided", command);
                return;
            } else if parameters.len() > 1 {
                println!("{}: too many parameters provided", command);
                return;
            }

            // Parse the parameter to an integer.
            let exit_value: i32 = match parameters[0].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}: must provide a number", command);
                    return;
                }
            };

            // Now exit with provided exit value.
            std::process::exit(exit_value)
        }
        "echo" => println!("{}", parameters.join(" ")),
        _ => println!("{}: command not found", command),
    }
}
