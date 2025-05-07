use crate::executor::query_result::QueryResult;
use crate::executor::ExecutionResult;
use sqlparser::ast::{SetExpr, Statement};

use super::{ExecutionError, TABLES};

/// 执行查询语句
///
/// # Arguments
/// * `stmt` - SQL 语句
///
/// # Returns
/// * `ExecutionResult<()>` - 执行结果
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果解析 SQL 语句失败
/// * `ExecutionError::TableNotFound` - 如果表不存在
pub fn query(stmt: &Statement) -> ExecutionResult<()> {
    match stmt {
        Statement::Query(query) => match &*query.body {
            SetExpr::Select(select) => {
                let table_name = extract_table_name(&select.from[0].relation)?;

                let tables = TABLES.lock().unwrap();
                let table = tables.get(table_name);
                if table.is_none() {
                    return Err(ExecutionError::TableNotFound(table_name.to_string()));
                }

                let query_result =
                    QueryResult::from_table(table.unwrap(), &select.selection, &select.projection)?;
                println!("{}", query_result.display());

                Ok(())
            }
            _ => Err(ExecutionError::ParseError(
                "无法解析查询语句：不支持的查询类型".to_string(),
            )),
        },
        _ => Err(ExecutionError::ParseError(
            "无法解析查询语句：不是查询语句".to_string(),
        )),
    }
}

/// 提取表名
///
/// # Arguments
///
/// * `relation` - 表达式
///
/// # Returns
/// * `Result<&String, ExecutionError>` - 返回表名或错误
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果无法解析表名
/// * `ExecutionError::TableNotFound` - 如果表名不存在
fn extract_table_name(relation: &sqlparser::ast::TableFactor) -> Result<&String, ExecutionError> {
    let table_name = match relation {
        sqlparser::ast::TableFactor::Table { name, .. } => {
            if let Some(ident) = name.0.first() {
                &ident.as_ident().unwrap().value
            } else {
                return Err(ExecutionError::ParseError(
                    "无法解析 SELECT 语句：无法提取表名".to_string(),
                ));
            }
        }
        _ => {
            return Err(ExecutionError::ParseError(
                "无法解析 SELECT 语句：无法提取表名".to_string(),
            ));
        }
    };
    Ok(table_name)
}
