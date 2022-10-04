/// Copies a string, with formatting, to the systems clipboard
#[macro_export]
macro_rules! copy {
    ($($arg:tt)+) => {
            if ::logger::STATE.read().unwrap().should_copy {
                let text = format!($($arg)+);
                if !text.is_empty() {
                    log_debug!("Copied output to buffer");
                    ::logger::copy_to_clipboard(text);
                } else {
                    log_debug!("Nothing to copy");
                }
        }
    };
}

/// Simple wrapper around eprint!
#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => {
        if ::logger::STATE.read().unwrap().level != ::logger::LogLevel::Off {
            eprint!($($arg)*);
        }
    };
}

/// Simple wrapper around println!
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if ::logger::STATE.read().unwrap().level != ::logger::LogLevel::Off {
            println!($($arg)*);
        }
    };
}

/// Generic logger. Should not be used outside of this file
#[macro_export]
macro_rules! internal_log {
    ($level:expr, $($arg:tt)+) => {
        if ::logger::STATE.read().unwrap().level >= $level {
            log!("[{}] {}", $level, format!($($arg)+));
        }
    };
}

/// Simple info logger
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        internal_log!(::logger::LogLevel::Info, $($arg)+);
    };
}

/// Simple debug logger
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        internal_log!(::logger::LogLevel::Debug, $($arg)+);
    };
}

/// Simple trace logger
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        internal_log!(::logger::LogLevel::Trace, $($arg)+);
    };
}

/// Simple warning logger
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        internal_log!(::logger::LogLevel::Warn, $($arg)+);
    };
}

/// Simple error logger
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        internal_log!(::logger::LogLevel::Error, $($arg)+);
    };
}
