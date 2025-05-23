//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message
//! "Failed to read input line".

use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// This functions reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use cli_lib::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let mystdin = std::io::stdin();
    let mut reader = BufReader::new(mystdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_input_newline() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output, "Line should be 'test'");
    }

    #[test]
    fn test_read_input_empty_with_newline() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }
}
