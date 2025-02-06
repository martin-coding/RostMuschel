use crate::call::Call;

const BUILTINS: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

/// Checks for each argument if it is a builtin.
pub fn call(call: &Call) {
    for arg in call.arguments.iter() {
        if BUILTINS.contains(arg) {
            println!("{} is a builtin", arg);
        } else {
            println!("{}: not found ", arg);
        }
    }
}
