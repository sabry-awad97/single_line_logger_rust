import datetime
import logging


class LogLevel:
    DEBUG = 10
    INFO = 20
    WARNING = 30
    ERROR = 40


class Logger:
    def __init__(self, log_level_threshold=LogLevel.INFO, log_to_file=False, file_path='log.txt'):
        self.log_level_threshold = log_level_threshold
        self.file_path = file_path
        self.logger = logging.getLogger('Logger')
        self.logger.setLevel(logging.DEBUG)

        formatter = logging.Formatter(
            '%(asctime)s [%(levelname)s] [%(module)s] %(message)s')

        # Log to console
        console_handler = logging.StreamHandler()
        console_handler.setLevel(logging.DEBUG)
        console_handler.setFormatter(formatter)
        self.logger.addHandler(console_handler)

        # Log to file
        if log_to_file:
            file_handler = logging.FileHandler(file_path)
            file_handler.setLevel(logging.DEBUG)
            file_handler.setFormatter(formatter)
            self.logger.addHandler(file_handler)

    def log(self, level, message, module=None):
        if level < self.log_level_threshold:
            return

        module_name = module or '__main__'

        # Format the log message with a timestamp and module name
        timestamp = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        formatted_message = f'[{timestamp}] [{level_string(level)}] [{module_name}] {message}'

        # Log the message
        if level == LogLevel.DEBUG:
            self.logger.debug(formatted_message)
        elif level == LogLevel.INFO:
            self.logger.info(formatted_message)
        elif level == LogLevel.WARNING:
            self.logger.warning(formatted_message)
        elif level == LogLevel.ERROR:
            self.logger.error(formatted_message)

    def set_log_level_threshold(self, log_level_threshold):
        self.log_level_threshold = log_level_threshold

    def get_log_level_threshold(self):
        return self.log_level_threshold


# Helper function to convert a LogLevel to a string
def level_string(level):
    if level == LogLevel.DEBUG:
        return 'DEBUG'
    elif level == LogLevel.INFO:
        return 'INFO'
    elif level == LogLevel.WARNING:
        return 'WARNING'
    elif level == LogLevel.ERROR:
        return 'ERROR'


logger = Logger(
    log_level_threshold=LogLevel.INFO, log_to_file=True, file_path='log.txt')

logger.log(LogLevel.DEBUG, 'This message should not be logged', 'main')
logger.log(LogLevel.INFO, 'Starting application', 'main')
logger.log(LogLevel.WARNING, 'Low disk space', 'disk')
logger.log(LogLevel.ERROR, 'Failed to open file', 'file')
logger.log(LogLevel.INFO, 'Exiting application', 'main')

logger.set_log_level_threshold(LogLevel.WARNING)
logger.log(LogLevel.INFO, 'This message should not be logged', 'main')
logger.log(LogLevel.WARNING, 'Connection lost', 'network')
logger.log(LogLevel.ERROR, 'Disk failure', 'disk')
