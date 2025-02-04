use crate::call::Call;

/// Exits the shell process with provided error code.
/// If no argument is provided, use code 0.
/// If more than one argument is provided, print an error message without exiting.
pub fn call(call: &Call) {
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
