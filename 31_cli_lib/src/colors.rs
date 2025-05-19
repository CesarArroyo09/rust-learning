//! ANSI color codes for use in terminal output. With helper functions to wrap strings in color codes.
//! # Examples:
//! ```
//! use cli_lib::colors::{red, blue};
//! let red_string = red("This is a red string");
//! let blue_string = blue("This is a blue string");
//! ```

/// Returns a string wrapped in ANSI red color codes.
/// # Examples:
/// ```
/// use cli_lib::colors::red;
/// let red_string = red("This is a red string");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
}

impl ColorString {
    // create method that will use the string and color fields to create a colorized version
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        }
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
