pub mod expr_evaluator;
pub mod query_processor;

use colored::Colorize;
use lazy_static::lazy_static;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    pub static ref IS_INFO_OUTPUT: AtomicBool = AtomicBool::new(true);
}

pub fn log_error(msg: impl Display) {
    eprintln!("{} {}", "[ERROR]".red(), msg.to_string().red());
}

pub fn log_warning(msg: impl Display) {
    eprintln!("{} {}", "[WARNING]".yellow(), msg);
}

pub fn log_info(msg: impl Display) {
    if !IS_INFO_OUTPUT.load(Ordering::Relaxed) {
        return;
    }
    println!("{} {}", "[INFO]".green(), msg);
}
