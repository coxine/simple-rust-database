use crate::executor::{ExecutionError, ExecutionResult};
use sqlparser::ast::{Expr, SetExpr, Statement, Value, Values};

fn extract_row_values(expr: &Expr) -> String {
    match expr {
        Expr::Value(val) => match &val.value {
            Value::SingleQuotedString(s) => s.clone(),
            Value::Number(n, _) => n.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "NULL".to_string(),
            _ => val.to_string(),
        },
        _ => expr.to_string(),
    }
}

fn extract_rows_to_insert(values: &Values) -> Vec<Vec<String>> {
    let mut rows_to_insert = Vec::new();

    for row in &values.rows {
        let row_values: Vec<String> = row.iter().map(extract_row_values).collect();
        rows_to_insert.push(row_values);
    }

    rows_to_insert
}

pub fn insert(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Insert(insert_stmt) = stmt {
        let table_name = insert_stmt.table.to_string();
        let columns_to_insert: Vec<String> = insert_stmt
            .columns
            .iter()
            .map(|col| col.value.clone())
            .collect();

        let rows_to_insert = match insert_stmt.source.as_ref().unwrap().body.as_ref() {
            SetExpr::Values(values) => extract_rows_to_insert(values),
            _ => {
                return Err(ExecutionError::ParseError(
                    "无法解析 INSERT 语句".to_string(),
                ));
            }
        };

        println!("INSERT: 表 '{}' 插入数据", table_name);
        println!("Column: {:?}", columns_to_insert);
        println!("Rows: {:?}", rows_to_insert);
    } else {
        return Err(ExecutionError::ParseError(
            "无法解析 INSERT 语句".to_string(),
        ));
    }
    Ok(())
}
