/// SQL 执行器模块
///
/// 负责执行 SQL 语句，包含各种 SQL 命令的处理逻辑，
/// 如创建表、插入数据、查询、更新和删除等操作。
use error::ExecutionResult;
use lazy_static::lazy_static;
use sqlparser::ast::Statement;
use std::collections::HashMap;
use std::sync::Mutex;
mod create_table;
mod delete;
mod drop;
pub mod error;
mod insert;
mod query;
mod query_result;
pub mod storage;
pub mod table;
mod update;

use crate::executor::table::Table;

pub use error::ExecutionError;

lazy_static! {
    pub static ref TABLES: Mutex<HashMap<String, Table>> = Mutex::new(HashMap::new());
    pub static ref EXECUTOR_INPUT: Mutex<String> = Mutex::new("".to_string());
}

/// 执行 SQL 语句
///
/// 根据语句类型分发到不同的处理函数。
///
/// # Arguments
///
/// * `stmt` - 要执行的 SQL 语句
/// * `input` - 整个 SQL 语句的输入字符串，供 OJ 评测使用
///
/// # Returns
///
/// * `ExecutionResult<()>` - 执行结果
pub fn execute_statement(stmt: &Statement, input: &str) -> ExecutionResult<()> {
    *EXECUTOR_INPUT.lock().unwrap() = input.to_string();
    match stmt {
        Statement::Query(_) => query::query(stmt),
        Statement::CreateTable { .. } => create_table::create_table(stmt),
        Statement::Drop { .. } => drop::drop(stmt),
        Statement::Insert { .. } => insert::insert(stmt),
        Statement::Delete { .. } => delete::delete(stmt),
        Statement::Update { .. } => update::update(stmt),
        _ => Err(ExecutionError::ExecutionError("未识别的命令".to_string())),
    }
}
