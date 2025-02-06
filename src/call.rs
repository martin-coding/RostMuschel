/// A parsed command call: a command plus its arguments.
///
/// The command and arguments are borrowed from the original input.
pub struct Call<'a> {
    pub command: &'a str,
    pub arguments: Vec<&'a str>,
}

impl<'a> Call<'a> {
    /// Create a new `Call` from an input string.
    ///
    /// The first token is taken as the command, and the rest are the arguments.
    /// Tokens that begin with a quote will only have the quotes removed if a matching
    /// closing quote is found.
    pub fn from_input(input: &'a str) -> Self {
        let tokens = parse_tokens(input);
        let mut iter = tokens.into_iter();
        let command = iter.next().unwrap_or("");
        let arguments: Vec<&'a str> = iter.collect();
        Call { command, arguments }
    }
}

/// Parse the input string into tokens (as slices of the input), handling single
/// and double quotes.
///
/// If a token begins with a quote and a matching closing quote
/// is found, the token is returned without the quotes. Otherwise, the token is
/// returned as-is.
fn parse_tokens(input: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let input_len = input.len();
    let input_bytes = input.as_bytes();
    let mut i = 0;
    // Helper to check for whitespace.
    let is_whitespace = |b: u8| b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
    while i < input_len {
        // Skip any leading whitespace.
        while i < input_len && is_whitespace(input_bytes[i]) {
            i += 1;
        }
        if i >= input_len {
            break;
        }
        let start = i;
        let current_char = input[i..].chars().next().unwrap();
        if current_char == '"' || current_char == '\'' {
            // This token starts with a quote.
            let quote = current_char;
            let orig_i = i; // remember where the quoted token started
            i += quote.len_utf8(); // move past the opening quote
            let token_content_start = i;
            // Search for the matching closing quote.
            let mut found_closing = false;
            while i < input_len {
                let ch = input[i..].chars().next().unwrap();
                if ch == quote {
                    found_closing = true;
                    break;
                }
                i += ch.len_utf8();
            }
            if found_closing {
                // If we found a matching closing quote, use the content inside the quotes.
                let token = &input[token_content_start..i];
                tokens.push(token);
                i += quote.len_utf8(); // move past the closing quote
            } else {
                // No matching closing quote was found.
                // Return the entire token including the opening quote.
                let token = &input[orig_i..i];
                tokens.push(token);
            }
        } else {
            // Unquoted token: read until the next whitespace.
            while i < input_len {
                let ch = input[i..].chars().next().unwrap();
                if is_whitespace(ch as u8) {
                    break;
                }
                i += ch.len_utf8();
            }
            let token = &input[start..i];
            tokens.push(token);
        }
    }
    tokens
}
