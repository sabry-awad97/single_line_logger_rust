use std::io::{self, Write};

// Enum defining the possible log levels
enum LogLevel {
    Info,
    Warning,
    Error,
}

// Struct defining a log message
struct LogMessage {
    level: LogLevel,
    message: String,
}

// Struct implementing a single-line logger
struct SingleLineLogger<W: Write> {
    stdout: W,
}

impl<W: Write> SingleLineLogger<W> {
    // Method to create a new instance of SingleLineLogger
    fn new(stdout: W) -> Self {
        SingleLineLogger { stdout }
    }

    // Method to log a message at the specified level
    fn log(&mut self, level: LogLevel, message: &str) -> io::Result<()> {
        // Move the cursor up one line and clear the line
        write!(self.stdout, "\x1B[1F\x1B[0J")?;

        // Format the log message
        let log_message = format!("[{}] {}", level_string(level), message);

        // Write the log message to the console
        write!(self.stdout, "{}", log_message)?;

        // Flush the output immediately
        self.stdout.flush()?;

        Ok(())
    }
}

// Helper function to convert a LogLevel to a string
fn level_string(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    }
}

fn main() -> io::Result<()> {
    // Create a new SingleLineLogger instance with stdout as the output
    let mut logger = SingleLineLogger::new(io::stdout());

    // Log some messages at different log levels
    logger.log(LogLevel::Info, "Starting program...")?;
    logger.log(LogLevel::Warning, "Something unusual happened.")?;
    logger.log(LogLevel::Error, "An error occurred.")?;

    // Signal that the program has completed successfully
    Ok(())
}
