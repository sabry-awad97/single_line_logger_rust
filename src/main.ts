import { format } from 'date-fns';
import fs from 'fs';

enum LogLevel {
  Debug,
  Info,
  Warning,
  Error,
}

interface LogMessage {
  level: LogLevel;
  module?: string;
  timestamp: string;
  message: string;
}

class SingleLineLogger {
  private output_stream: NodeJS.WritableStream;
  private log_level_threshold: LogLevel;
  private file_logger?: fs.WriteStream;

  constructor(
    output_stream: NodeJS.WritableStream,
    log_level_threshold: LogLevel,
    log_to_file: boolean,
    file_path?: string
  ) {
    this.output_stream = output_stream;
    this.log_level_threshold = log_level_threshold;

    if (log_to_file) {
      const path = file_path ?? 'log.txt';
      this.file_logger = fs.createWriteStream(path, { flags: 'a' });
    }
  }

  private levelString(level: LogLevel): string {
    switch (level) {
      case LogLevel.Debug:
        return 'DEBUG';
      case LogLevel.Info:
        return 'INFO';
      case LogLevel.Warning:
        return 'WARNING';
      case LogLevel.Error:
        return 'ERROR';
    }
  }

  private formatMessage(
    level: LogLevel,
    message: string,
    module?: string
  ): LogMessage {
    const timestamp = format(new Date(), 'yyyy-MM-dd HH:mm:ss');
    const formattedMessage: LogMessage = {
      level,
      timestamp,
      message,
    };

    if (module) {
      formattedMessage.module = module;
    }

    return formattedMessage;
  }

  log(level: LogLevel, message: string, module?: string): void {
    if (level > this.log_level_threshold) {
      return;
    }

    // Move the cursor up one line and clear the line
    this.output_stream.write('\x1B[1F\x1B[0J');

    // Format the log message with a timestamp and module name
    const formattedMessage = this.formatMessage(level, message, module);

    // Write the log message to the output stream and file logger (if enabled)
    const messageStr = this.formatMessageString(formattedMessage);
    this.output_stream.write(messageStr);
    if (this.file_logger) {
      this.file_logger.write(messageStr);
    }
  }

  setLogLevelThreshold(log_level_threshold: LogLevel): void {
    this.log_level_threshold = log_level_threshold;
  }

  getLogLevelThreshold(): LogLevel {
    return this.log_level_threshold;
  }

  private formatMessageString(message: LogMessage): string {
    const moduleStr = message.module ? `[${message.module}] ` : '';
    return `[${message.timestamp}] [${this.levelString(
      message.level
    )}] ${moduleStr}${message.message}\n`;
  }

  close(): void {
    if (this.file_logger) {
      this.file_logger.end();
    }
  }
}

// Example usage:
const logger = new SingleLineLogger(
  process.stdout,
  LogLevel.Info,
  true,
  'log.txt'
);

// Log some messages
logger.log(LogLevel.Debug, 'This message should not be logged', 'main');
logger.log(LogLevel.Info, 'Starting application', 'main');
logger.log(LogLevel.Warning, 'Low disk space', 'disk');
logger.log(LogLevel.Error, 'Failed to open file', 'file');
logger.log(LogLevel.Info, 'Exiting application', 'main');

// Set the log level threshold to warning and log some more messages
logger.setLogLevelThreshold(LogLevel.Warning);
logger.log(LogLevel.Info, 'This message should not be logged', 'main');
logger.log(LogLevel.Warning, 'Connection lost', 'network');
logger.log(LogLevel.Error, 'Disk failure', 'disk');
