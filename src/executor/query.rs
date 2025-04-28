use crate::executor::table::{Column, ColumnDataType, QueryResult};
use crate::executor::ExecutionResult;
use sqlparser::ast::{SelectItem, SetExpr, Statement};

use super::table::Table;
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
                let relation = &select.from[0].relation;
                let table_name = extract_table_name(relation)?;

                let tables = TABLES.lock().unwrap();
                let table = tables.get(table_name);
                if table.is_none() {
                    return Err(ExecutionError::TableNotFound(table_name.to_string()));
                }

                let column_index =
                    extract_column_index(select.projection.clone(), &table.unwrap())?;
                let query_result = QueryResult::from_table(table.unwrap(), column_index);
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

/// 提取列索引
///
/// # Arguments
/// * `projection` - 投影项列表
/// * `table` - 表对象
///
/// # Returns
/// * `Result<Option<Vec<usize>>, ExecutionError>` - 返回列索引列表或错误
/// * `None` - 如果投影项是通配符
/// * `Some(Vec<usize>)` - 如果投影项是具体列名
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果无法解析投影项
/// * `ExecutionError::ColumnNotFound` - 如果列名不存在
fn extract_column_index(
    projection: Vec<SelectItem>,
    table: &Table,
) -> Result<Option<Vec<usize>>, ExecutionError> {
    if is_wildcard(&projection) {
        Ok(None)
    } else {
        let columns = &projection;
        let column_names = extract_column_names(columns)?;
        let column_index = column_names
            .iter()
            .map(|col| {
                table
                    .get_column_index(&col.name)
                    .ok_or_else(|| ExecutionError::ColumnNotFound(col.name.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(column_index))
    }
}

// 检查投影项是否是通配符
fn is_wildcard(items: &Vec<SelectItem>) -> bool {
    for item in items {
        match item {
            SelectItem::Wildcard(_) => return true,
            _ => continue,
        }
    }
    false
}

/// 提取列名
///
/// # Arguments
///
/// * `projection_items` - 投影项列表
///
/// # Returns
/// * `Result<Vec<Column>, ExecutionError>` - 返回列名列表或错误
///
/// # Errors
/// * `ExecutionError::ParseError` - 如果无法解析投影项
/// * `ExecutionError::ColumnNotFound` - 如果列名不存在
fn extract_column_names(projection_items: &Vec<SelectItem>) -> Result<Vec<Column>, ExecutionError> {
    let mut columns = Vec::new();

    for item in projection_items {
        match item {
            SelectItem::UnnamedExpr(expr) => {
                // 处理表达式，尝试提取列名
                if let sqlparser::ast::Expr::Identifier(ident) = expr {
                    columns.push(Column {
                        name: ident.value.clone(),
                        data_type: ColumnDataType::Varchar(None), // 暂时使用默认类型
                        is_primary_key: false,
                        is_nullable: true,
                    });
                } else {
                    return Err(ExecutionError::ParseError(
                        "无法解析 SELECT 语句：不支持的表达式".to_string(),
                    ))?;
                }
            }
            _ => {
                Err(ExecutionError::ParseError(
                    "无法解析 SELECT 语句：不支持的投影项".to_string(),
                ))?;
            }
        }
    }
    Ok(columns)
}
