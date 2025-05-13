/// 工具函数模块
///
/// 包含系统中使用的通用工具函数和子模块，如表达式求值、日志记录等。
pub mod expr_evaluator;
pub mod query_processor;

use colored::Colorize;
use lazy_static::lazy_static;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    pub static ref IS_INFO_OUTPUT: AtomicBool = AtomicBool::new(true);
}

/// 输出错误日志
///
/// 将错误消息以红色输出到标准错误流。
///
/// # Arguments
///
/// * `msg` - 要输出的错误消息
pub fn log_error(msg: impl Display) {
    eprintln!("{} {}", "[ERROR]".red(), msg.to_string().red());
}

/// 输出警告日志
///
/// 将警告消息以黄色输出到标准错误流。
///
/// # Arguments
///
/// * `msg` - 要输出的警告消息
pub fn log_warning(msg: impl Display) {
    eprintln!("{} {}", "[WARNING]".yellow(), msg);
}

/// 输出信息日志
///
/// 如果启用了信息输出，则将信息消息以绿色输出到标准输出流。
///
/// # Arguments
///
/// * `msg` - 要输出的信息消息
pub fn log_info(msg: impl Display) {
    if !IS_INFO_OUTPUT.load(Ordering::Relaxed) {
        return;
    }
    println!("{} {}", "[INFO]".green(), msg);
}
