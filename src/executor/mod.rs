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
}

pub fn execute_statement(stmt: &Statement) -> ExecutionResult<()> {
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
