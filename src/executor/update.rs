use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use sqlparser::ast::{ Statement, TableFactor};

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
        let tables = TABLES.lock().unwrap();
        if tables.contains_key(&table_name) == false {
            return Err(ExecutionError::TableNotFound(table_name));
        }
        let table = tables.get(&table_name).unwrap();
        let where_clause = selection.as_ref();
        
        // table.update_raws(assignments,where_clause);

    } else {
        return Err(ExecutionError::ParseError("无法解析UPDATE语句".to_string()));
    }
    Ok(())
}
