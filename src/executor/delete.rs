/// 删除操作模块
///
/// 实现 DELETE FROM 语句的解析和执行逻辑，负责从表中删除数据。
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use sqlparser::ast::{FromTable, Statement, TableFactor};

/// 执行删除操作
///
/// 解析 DELETE 语句，验证表是否存在，然后删除匹配条件的行。
///
/// # Arguments
///
/// * `stmt` - SQL 语句对象，预期为 DELETE 语句
///
/// # Returns
///
/// * `ExecutionResult<()>` - 删除操作的结果，成功或失败
pub fn delete(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Delete(delete) = stmt {
        let tables = match &(delete.from) {
            FromTable::WithFromKeyword(vec) => vec,
            FromTable::WithoutKeyword(vec) => vec,
        };
        for table in tables {
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
            let where_clause = &delete.selection;
            /*let limit = match &delete.limit {
                Some(exp) => match exp {
                    Expr::Value(val) => match &val.value {
                        Value::Number(num, _) => num.parse::<i64>().unwrap(),
                        _ => 0
                    },
                    _ => 0
                },
                None => 0,
            };*/

            table.delete_rows(where_clause)?
        }
    } else {
        return Err(ExecutionError::ParseError("无法解析DELETE语句".to_string()));
    }
    Ok(())
}
