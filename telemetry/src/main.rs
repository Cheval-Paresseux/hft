use telemetry::logging::{Logger, LogLevel, LogEvent};

fn main () {
    // 1. Init Logger
    let logger_name = "Logger1";
    let mut _logger = Logger::hot_scope(logger_name, None);
}