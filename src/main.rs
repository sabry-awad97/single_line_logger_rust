use chrono::Local;
use std::fs::OpenOptions;
use std::io::{self, Write};

// Enum defining the possible log levels
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Struct defining a log message
struct LogMessage {
    level: LogLevel,
    module: Option<String>,
    timestamp: String,
    message: String,
}

// Struct implementing a single-line logger
struct SingleLineLogger<W: Write> {
    output_stream: W,
    log_level_threshold: LogLevel,
    file_logger: Option<std::fs::File>,
}

impl<W: Write> SingleLineLogger<W> {
    // Method to create a new instance of SingleLineLogger
    fn new(
        output_stream: W,
        log_level_threshold: LogLevel,
        log_to_file: bool,
        file_path: Option<&str>,
    ) -> io::Result<Self> {
        let file_logger = if log_to_file {
            match file_path {
                Some(path) => Some(OpenOptions::new().create(true).append(true).open(path)?),
                None => Some(
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("log.txt")?,
                ),
            }
        } else {
            None
        };

        Ok(SingleLineLogger {
            output_stream,
            log_level_threshold,
            file_logger,
        })
    }

    // Method to log a message at the specified level
    fn log(&mut self, level: LogLevel, message: &str, module: Option<&str>) -> io::Result<()> {
        if level > self.log_level_threshold {
            return Ok(());
        }

        // Move the cursor up one line and clear the line
        write!(self.output_stream, "\x1B[1F\x1B[0J")?;

        // Format the log message with a timestamp and module name
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let formatted_message = match module {
            Some(module_name) => format!(
                "[{}] [{}] [{}] {}",
                timestamp,
                level_string(level),
                module_name,
                message
            ),
            None => format!("[{}] [{}] {}", timestamp, level_string(level), message),
        };

        // Write the log message to the output stream and file logger (if enabled)
        write!(self.output_stream, "{}", formatted_message)?;
        if let Some(file_logger) = &mut self.file_logger {
            writeln!(file_logger, "{}", formatted_message)?;
        }

        // Flush the output immediately
        self.output_stream.flush()?;
        if let Some(file_logger) = &mut self.file_logger {
            file_logger.flush()?;
        }

        Ok(())
    }

    // Method to set the log level threshold
    fn set_log_level_threshold(&mut self, log_level_threshold: LogLevel) {
        self.log_level_threshold = log_level_threshold;
    }

    // Method to get the current log level threshold
    fn get_log_level_threshold(&self) -> LogLevel {
        self.log_level_threshold
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
    // Create a new logger that logs messages at the info level or higher to the console and a file
    let mut logger = SingleLineLogger::new(io::stdout(), LogLevel::Info, true, Some("log.txt"))?;

    // Log some messages
    logger.log(
        LogLevel::Debug,
        "This message should not be logged",
        Some("main"),
    )?;
    logger.log(LogLevel::Info, "Starting application", Some("main"))?;
    logger.log(LogLevel::Warning, "Low disk space", Some("disk"))?;
    logger.log(LogLevel::Error, "Failed to open file", Some("file"))?;
    logger.log(LogLevel::Info, "Exiting application", Some("main"))?;

    // Set the log level threshold to warning and log some more messages
    logger.set_log_level_threshold(LogLevel::Warning);
    logger.log(
        LogLevel::Info,
        "This message should not be logged",
        Some("main"),
    )?;
    logger.log(LogLevel::Warning, "Connection lost", Some("network"))?;
    logger.log(LogLevel::Error, "Disk failure", Some("disk"))?;

    Ok(())
}
