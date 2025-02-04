use crate::call::Call;

/// Prints all provided arguments but replaces all whitespace with a single space.
pub fn call(call: &Call) {
    println!("{}", call.arguments.join(" "));
}
