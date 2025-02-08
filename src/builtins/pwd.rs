use std::env;

use crate::call::Call;

/// Prints the current working directory.
pub fn call(_call: &Call) {
    // Ignores all arguments

    let current_dir = env::current_dir().expect("Failed to get current working directory.");
    println!("{}", current_dir.display());
}
