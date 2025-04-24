use crate::executor::{ExecutionError, ExecutionResult};
use sqlparser::ast::Statement;
pub fn insert(stmt: &Statement) -> ExecutionResult<()> {
    println!("Insert: {:?}", stmt);
    // 临时返回 Ok，后续实现插入逻辑
    Ok(())
}
