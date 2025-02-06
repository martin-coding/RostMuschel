use std::str::SplitWhitespace;

/// Represent a command call with arguments.
pub struct Call<'a> {
    pub command: &'a str,
    pub arguments: Vec<&'a str>,
}

impl<'a> Call<'a> {
    /// Turns an input string into a command call.
    pub fn from_input(input: &'a str) -> Self {
        // Split the input into a command and arguments.
        let mut split: SplitWhitespace = input.split_whitespace();
        let command: &str = split.next().unwrap_or_default();
        let arguments: Vec<&str> = split.collect();
        Call { command, arguments }
    }
}
