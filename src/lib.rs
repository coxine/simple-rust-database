/// 简易的 Rust 数据库库
///
/// 提供了一个简单的基于内存的数据库，支持基本的 SQL 操作，
/// 以及数据持久化功能。
use std::sync::atomic::Ordering;

use utils::IS_INFO_OUTPUT;

pub mod executor;
pub mod model;
pub mod parser;
pub mod repl;
pub mod utils;

/// 执行 SQL 语句
///
/// 这是一个用于外部测试的函数，它会关闭信息输出，执行 SQL 语句，
/// 并返回执行是否成功。
///
/// # Arguments
///
/// * `sql_statement` - SQL 语句字符串
///
/// # Returns
///
/// 如果 SQL 语句执行成功，返回 `true`；否则返回 `false`
pub fn execute_sql(sql_statement: &str) -> bool {
    IS_INFO_OUTPUT.store(false, Ordering::Relaxed);
    match parser::parse_sql(sql_statement) {
        Ok(statements) => {
            for statement in statements {
                let execute_result = executor::execute_statement(&statement, sql_statement);
                if let Err(_) = execute_result {
                    return false;
                }
            }
        }
        Err(_) => {
            println!("Error: Syntax error");
            return false;
        }
    }
    true
}
