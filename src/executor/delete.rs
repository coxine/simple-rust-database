use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use sqlparser::ast::{ FromTable, Statement, TableFactor};

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
            let tables = TABLES.lock().unwrap();
            if tables.contains_key(&table_name) == false {
                return Err(ExecutionError::TableNotFound(table_name));
            }
            let _table = tables.get(&table_name).unwrap();
            let _where_clause = delete.selection.as_ref();
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

            // table.delete_rows(where_clause);
        }
    } else {
        return Err(ExecutionError::ParseError("无法解析DELETE语句".to_string()));
    }
    Ok(())
}
