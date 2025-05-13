/// 更新操作模块
///
/// 实现 UPDATE 语句的解析和执行逻辑，负责更新表中的数据。
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use sqlparser::ast::{Statement, TableFactor};

/// 执行更新操作
///
/// 解析 UPDATE 语句，验证表是否存在，然后更新匹配条件的行。
///
/// # Arguments
///
/// * `stmt` - SQL 语句对象，预期为 UPDATE 语句
///
/// # Returns
///
/// * `ExecutionResult<()>` - 更新操作的结果，成功或失败
pub fn update(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Update {
        table,
        assignments,
        selection,
        ..
    } = stmt
    {
        let table_name = match &table.relation {
            TableFactor::Table { name, .. } => name.to_string(),
            _ => {
                return Err(ExecutionError::ExecutionError(
                    "暂时无法解析的表名".to_string(),
                ));
            }
        };
        let mut tables = TABLES.lock().unwrap();
        if tables.contains_key(&table_name) == false {
            return Err(ExecutionError::TableNotFound(table_name));
        }
        let table = tables.get_mut(&table_name).unwrap();
        let where_clause = selection;

        table.update_rows(assignments, where_clause)?;
    } else {
        return Err(ExecutionError::ParseError("无法解析UPDATE语句".to_string()));
    }
    Ok(())
}
