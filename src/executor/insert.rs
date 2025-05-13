/// 数据插入操作模块
///
/// 实现 INSERT INTO 语句的解析和执行逻辑，负责向表中插入数据行。
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use crate::model::Value as TableValue;
use crate::utils;
use sqlparser::ast::{Expr, SetExpr, Statement, Value, Values};

fn extract_row_values(expr: &Expr) -> TableValue {
    match expr {
        Expr::Value(val) => match &val.value {
            Value::SingleQuotedString(s) => TableValue::Varchar(s.clone()),
            Value::DoubleQuotedString(s) => TableValue::Varchar(s.clone()),
            Value::Number(n, _) => TableValue::Int(n.parse::<i64>().unwrap()),
            Value::Null => TableValue::Null,
            _ => TableValue::Varchar(val.to_string()),
        },
        Expr::Identifier(ident) => TableValue::Varchar(ident.value.clone()),
        _ => TableValue::Varchar(expr.to_string()),
    }
}

fn extract_rows_to_insert(values: &Values) -> Vec<Vec<TableValue>> {
    let mut data_to_insert = Vec::new();

    for row in &values.rows {
        let row_values: Vec<TableValue> = row.iter().map(extract_row_values).collect();
        data_to_insert.push(row_values);
    }

    data_to_insert
}

/// 重新排序插入数据，使其与表结构列顺序一致
///
/// 如果指定了列名(column_names非空)，则按照表的列顺序重排数据
/// 如果未指定列名，则直接使用数据行的顺序
/// 对于未提供值的列，将使用`Null`值填充
fn reorder_insert_data(
    table_name: &str,
    column_names: &[String],
    data_rows: Vec<Vec<TableValue>>,
) -> ExecutionResult<Vec<Vec<TableValue>>> {
    if column_names.is_empty() {
        return Ok(data_rows);
    }

    let tables = TABLES.lock().unwrap();
    let table = match tables.get(table_name) {
        Some(t) => t,
        None => return Err(ExecutionError::TableNotFound(table_name.to_string())),
    };

    let table_columns: Vec<String> = table.columns.iter().map(|col| col.name.clone()).collect();

    // 检查用户提供的列是否都存在于表中
    for col in column_names {
        if !table_columns.contains(col) {
            return Err(ExecutionError::ExecutionError(format!(
                "列 '{}' 在表 '{}' 中不存在",
                col, table_name
            )));
        }
    }

    let mut reordered_rows = Vec::new();
    for row in data_rows {
        if row.len() != column_names.len() {
            return Err(ExecutionError::ExecutionError(format!(
                "插入数据列数 ({}) 与指定列名数量 ({}) 不匹配",
                row.len(),
                column_names.len()
            )));
        }

        // 创建一个映射，将用户提供的列和对应的值关联起来
        let mut column_value_map = std::collections::HashMap::new();
        for (i, col_name) in column_names.iter().enumerate() {
            column_value_map.insert(col_name, row[i].clone());
        }

        // 按照表的列顺序创建新的数据行
        let mut new_row: Vec<TableValue> = Vec::new();
        for table_col in &table_columns {
            match column_value_map.get(table_col) {
                Some(val) => new_row.push(val.clone()),
                None => new_row.push(TableValue::Null), // 对于未提供的列，使用Null值
            }
        }

        reordered_rows.push(new_row);
    }

    Ok(reordered_rows)
}

/// 执行插入操作
///
/// 解析 INSERT INTO 语句，验证表是否存在和值是否有效，然后插入行数据。
///
/// # Arguments
///
/// * `stmt` - SQL 语句对象，预期为 INSERT 语句
///
/// # Returns
///
/// * `ExecutionResult<()>` - 插入操作的结果，成功或失败
pub fn insert(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Insert(insert_stmt) = stmt {
        let table_name = insert_stmt.table.to_string();
        let column_names: Vec<String> = insert_stmt
            .columns
            .iter()
            .map(|col| col.value.clone())
            .collect();

        let data_to_insert = match insert_stmt.source.as_ref().unwrap().body.as_ref() {
            SetExpr::Values(values) => extract_rows_to_insert(values),
            _ => {
                return Err(ExecutionError::ParseError(
                    "无法解析 INSERT 语句".to_string(),
                ));
            }
        };

        // 如果提供了列名，重新排序数据以匹配表结构
        let ordered_data = reorder_insert_data(&table_name, &column_names, data_to_insert)?;

        let mut tables = TABLES.lock().unwrap();
        let table = tables.get_mut(&table_name);
        if let Some(table) = table {
            for row in ordered_data {
                table.insert_row(row)?;
                utils::log_info(format!("INSERT: 成功插入到表 {}", table_name));
            }
        } else {
            return Err(ExecutionError::TableNotFound(table_name));
        }
    }
    Ok(())
}
