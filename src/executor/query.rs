use crate::executor::ExecutionResult;
use sqlparser::ast::Statement;
pub fn query(stmt: &Statement) -> ExecutionResult<()> {
    println!("Query: {:?}", stmt);
    // 临时返回 Ok，后续实现查询逻辑
    Ok(())
}
