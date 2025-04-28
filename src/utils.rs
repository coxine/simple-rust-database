use colored::Colorize;
use std::fmt::Display;

pub fn log_error(msg: impl Display) {
    eprintln!("{} {}", "[ERROR]".red(), msg.to_string().red());
}

pub fn log_warning(msg: impl Display) {
    eprintln!("{} {}", "[WARNING]".yellow(), msg);
}

pub fn log_info(msg: impl Display) {
    println!("{} {}", "[INFO]".green(), msg);
}
