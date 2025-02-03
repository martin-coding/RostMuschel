use std::io;
use std::io::Write;

// The character that is printed on every new line.
const SHELL_CHAR: char = '>';

fn main() {
    loop {
        // Prints the shell character and a whitespace.
        print!("{} ", SHELL_CHAR);
        io::stdout().flush().unwrap();

        // Get user input.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // TODO: logic
        println!("{}: command not found", input.trim());
    }
}
