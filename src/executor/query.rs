use crate::executor::query_result::QueryResult;
use crate::executor::ExecutionResult;
use sqlparser::ast::{SetExpr, Statement};

use super::{ExecutionError, TABLES};

/// 执行查询语句
/// 处理 SQL 查询语句并输出结果。支持标准 SELECT 查询，包含表查询和无表查询。
///
/// # Arguments
/// * `stmt` - SQL 语句，表示要执行的查询
///
/// # Returns
/// * `ExecutionResult<()>` - 执行结果，成功时返回 Ok(())
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果解析 SQL 语句失败
/// * `ExecutionError::TableNotFound` - 如果查询的表不存在
///
pub fn query(stmt: &Statement) -> ExecutionResult<()> {
    match stmt {
        Statement::Query(query) => match &*query.body {
            SetExpr::Select(select) => {
                // 处理无表查询，比如 SELECT 1+1
                if select.from.is_empty() {
                    let query_result = QueryResult::from_table(
                        None,
                        &select.selection,
                        &select.projection,
                        &query.order_by,
                    )?;
                    println!("{}", query_result.display());
                    return Ok(());
                }

                // 处理有表的查询
                let table_name = extract_table_name(&select.from[0].relation)?;
                let tables = TABLES.lock().unwrap();
                let table = tables.get(table_name);
                if table.is_none() {
                    return Err(ExecutionError::TableNotFound(table_name.to_string()));
                }
                let query_result = QueryResult::from_table(
                    table,
                    &select.selection,
                    &select.projection,
                    &query.order_by,
                )?;
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

/// 从 SQL 表达式中提取表名
///
/// 从给定的 TableFactor 对象中提取出表名字符串，主要用于 SELECT 查询中的 FROM 子句处理。
///
/// # Arguments
///
/// * `relation` - 表达式，代表 SQL 查询中的表引用
///
/// # Returns
/// * `Result<&String, ExecutionError>` - 成功时返回表名的引用，失败时返回错误
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果无法从表达式中解析出有效的表名
/// * `ExecutionError::TableNotFound` - 如果解析出的表名在数据库中不存在
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
