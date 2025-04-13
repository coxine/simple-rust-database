use sqlparser::ast::Statement;
mod create_table;
mod drop;
pub mod error;

pub use error::{ExecutionError, ExecutionResult};

pub fn execute_statement(stmt: &Statement) -> ExecutionResult<()> {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table::create_csv_table(stmt),
        Statement::Drop { .. } => drop::drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete(stmt),
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

fn delete(stmt: &Statement) -> ExecutionResult<()> {
    println!("Delete: {:?}", stmt);
    // 临时返回 Ok，后续实现删除逻辑
    Ok(())
}

fn update(stmt: &Statement) -> ExecutionResult<()> {
    println!("Update: {:?}", stmt);
    // 临时返回 Ok，后续实现更新逻辑
    Ok(())
}
