use std::io::{self, Write};
use chrono::{Local, TimeZone};

// Enum defining the possible log levels
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Struct defining a log message
struct LogMessage {
    level: LogLevel,
    timestamp: String,
    message: String,
}

// Struct implementing a single-line logger
struct SingleLineLogger<W: Write> {
    output_stream: W,
}

impl<W: Write> SingleLineLogger<W> {
    // Method to create a new instance of SingleLineLogger
    fn new(output_stream: W) -> Self {
        SingleLineLogger { output_stream }
    }

    // Method to log a message at the specified level
    fn log(&mut self, level: LogLevel, message: &str) -> io::Result<()> {
        // Move the cursor up one line and clear the line
        write!(self.output_stream, "\x1B[1F\x1B[0J")?;

        // Format the log message with a timestamp
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let formatted_message = format!("[{}] [{}] {}", timestamp, level_string(level), message);

        // Write the log message to the output stream
        write!(self.output_stream, "{}", formatted_message)?;

        // Flush the output immediately
        self.output_stream.flush()?;

        Ok(())
    }
}

// Helper function to convert a LogLevel to a string
fn level_string(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    }
}

fn main() -> io::Result<()> {
    // Create a new SingleLineLogger instance with stdout as the output stream
    let mut logger = SingleLineLogger::new(io::stdout());

    // Log some messages at different log levels
    logger.log(LogLevel::Info, "Starting program...")?;
    logger.log(LogLevel::Warning, "Something unusual happened.")?;
    logger.log(LogLevel::Error, "An error occurred.")?;

    // Signal that the program has completed successfully
    Ok(())
}
