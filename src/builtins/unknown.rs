use std::env;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use crate::call::Call;

/// Called if the given command is not a builtin.
///
/// Tries to execute the provided command.
/// It should be either a path to an executable or an executable in PATH.
///
/// Prints an error message if it fails.
pub fn call(call: &Call) {
    // Use own command string with replaced ~ instead of call.command.
    let mut command: String;

    // Replace ~ or ~/ at the beginning with $HOME, because Path::canonicalize() will not do it.
    if call.command == "~" || call.command.starts_with("~/") {
        command = env::var("HOME").expect("Failed to get HOME variable.");
        command.push_str(&call.command[1..]);
    } else {
        command = call.command.to_owned();
    }

    // Check if path is a directory.
    // Necessary because error-catching on Command::spawn() provides ErrorKind::PermissionDenied
    // instead of ErrorKind::IsADirectory.
    if let Ok(path) = Path::new(&command).canonicalize() {
        if path.is_dir() {
            eprintln!("{}: is a directory", command);
            return;
        }
    }

    // Spawn a child process for the given command.
    let mut child = match Command::new(&command).args(&call.arguments).spawn() {
        Ok(child) => child,
        Err(err) => {
            // Match used instead of only .kind().to_string() to change a few default descriptions
            // and make them fixed to future changes.
            let err_message = match err.kind() {
                ErrorKind::NotFound => "not found",
                ErrorKind::PermissionDenied => "permission denied",
                ErrorKind::NotSeekable => "file is not seekable",
                _ => &err.kind().to_string(),
            };

            eprintln!("{}: {}", command, err_message);
            return;
        }
    };

    // Wait for child to finish execution.
    child.wait().expect("Failed to wait on child");
}
