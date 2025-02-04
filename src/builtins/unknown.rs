use crate::call::Call;

/// Prints an error message.
pub fn call(call: &Call) {
    println!("{}: command not found", call.command);
}
