use lazy_static::lazy_static;
use sqlparser::ast::Statement;
use std::collections::HashMap;
use std::sync::Mutex;
mod create_table;
mod delete;
mod drop;
pub mod error;
pub mod storage;
mod types;
use crate::executor::types::Table;

pub use error::{ExecutionError, ExecutionResult};

lazy_static! {
    pub static ref TABLES: Mutex<HashMap<String, Table>> = Mutex::new(HashMap::new());
}

pub fn execute_statement(stmt: &Statement) -> ExecutionResult<()> {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table::create_table(stmt),
        Statement::Drop { .. } => drop::drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete::delete(stmt),
        Statement::Update { .. } => update(stmt),
        _ => Err(ExecutionError::ExecutionError("未识别的命令".to_string())),
    }
}

fn query(stmt: &Statement) -> ExecutionResult<()> {
    println!("Query: {:?}", stmt);
    // 临时返回 Ok，后续实现查询逻辑
    Ok(())
}

fn insert(stmt: &Statement) -> ExecutionResult<()> {
    println!("Insert: {:?}", stmt);
    // 临时返回 Ok，后续实现插入逻辑
    Ok(())
}

fn update(stmt: &Statement) -> ExecutionResult<()> {
    println!("Update: {:?}", stmt);
    // 临时返回 Ok，后续实现更新逻辑
    Ok(())
}
