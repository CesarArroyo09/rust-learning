//! This module contains the configuration options for the application
//!

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains the configuration options for controlling logging.
/// # ExampleS:
/// ```
/// use cli_lib::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Create a new `Logging` struct with custom values:
/// ```
/// use cli_lib::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File("log.txt".to_string()),
/// };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}

// fn example() {
//     let config = Logging::new();
//     println!("Logging enabled: {}", config.enabled);
// }
