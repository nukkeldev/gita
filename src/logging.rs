use colored::Colorize;

pub fn error(msg: &str) -> String {
    format!("{} {}", "[ERROR]".on_red(), msg)
}

pub fn yn(msg: &str) -> String {
    format!("{} [{}/{}] ", msg, "Y".green(), "N".red())
}

pub fn success(msg: &str) -> String {
    format!("{} {}", "[SUCCESS]".on_green(), msg)
}
pub fn warning(msg: &str) -> String {
    format!("{} {}", "[WARNING]".yellow(), msg)
}

pub fn log(msg: &str) -> String {
    format!("{} {}", "[LOG]".on_blue(), msg)
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        print!("{}", logging::error($msg));
    };
}

#[macro_export]
macro_rules! yn {
    ($msg:expr) => {
        print!("{}", logging::yn($msg))
    };
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        print!("{}", logging::success($msg));
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        print!("{}", logging::warning($msg));
    };
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        print!("{}", logging::log($msg));
    };
}
