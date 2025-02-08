use std::env;
use std::path::Path;

use crate::call::Call;

/// Change working directory.
/// If no argument is provided, go to home.
/// If more than one argument is provided, print an error message.
pub fn call(call: &Call) {
    // Check if correct number of arguments provided.
    if call.arguments.is_empty() {
        // Go to home if no arguments provided.
        change_directory("~");
        return;
    } else if call.arguments.len() > 1 {
        println!("{}: too many arguments provided", call.command);
        return;
    }

    change_directory(call.arguments[0]);
}

fn change_directory(directory: &str) {
    // Handle tilde case because set_current_dir throws error otherwise.
    if directory.starts_with('~') {
        let home_dir = env::var("HOME").expect("Failed to get HOME variable.");
        let directory = directory.replace("~", &home_dir);
        let path = Path::new(&directory);
        env::set_current_dir(path).expect("Failed to change directory.");
        return;
    }

    let path = Path::new(directory);
    env::set_current_dir(path).expect("Failed to change directory.");
}
