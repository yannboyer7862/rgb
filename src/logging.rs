pub enum LogType {
    Error,
    Warning,
    Info,
}

pub fn logger_log(log_type: LogType, message: &str) {
    match log_type {
        LogType::Error => eprintln!("[ERROR] {}", message),
        LogType::Warning => eprintln!("[WARNING] {}", message),
        LogType::Info => eprintln!("[INFO] {}", message),
    }
}