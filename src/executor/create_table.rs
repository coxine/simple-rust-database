use sqlparser::ast::{CharacterLength, CreateTable, DataType, Statement};

use crate::executor::table::Table;
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use crate::model::{Column, ColumnDataType as TableDataType};
use crate::utils;

pub fn create_table(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::CreateTable(create_table_stmt) = stmt {
        let table_name = create_table_stmt
            .name
            .0
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<String>>()
            .join("_");

        let mut tables = TABLES.lock().unwrap();
        if tables.contains_key(&table_name) {
            return Err(ExecutionError::TableExists(table_name));
        }

        let columns = create_table_columns(create_table_stmt);
        let table = Table::new(table_name.clone(), columns);

        tables.insert(table_name.clone(), table);
        utils::log_info(format!("CREATE: 表 '{}' 创建成功", table_name));
        Ok(())
    } else {
        Err(ExecutionError::ParseError(
            "无法解析 CREATE 语句".to_string(),
        ))
    }
}

fn create_table_columns(create_table_stmt: &CreateTable) -> Vec<Column> {
    create_table_stmt
        .columns
        .iter()
        .map(|col| {
            let data_type = match &col.data_type {
                DataType::Varchar(opt) => {
                    let length = match opt {
                        Some(CharacterLength::IntegerLength { length, .. }) => Some(*length),
                        Some(CharacterLength::Max) => None, // Handle MAX as None (unlimited)
                        None => None,
                    };
                    TableDataType::Varchar(length)
                }
                DataType::Int(opt) => TableDataType::Int(opt.clone()),
                _ => TableDataType::Varchar(None),
            };

            let is_primary_key = col.options.iter().any(|opt| {
                matches!(
                    opt.option,
                    sqlparser::ast::ColumnOption::Unique {
                        is_primary: true,
                        ..
                    }
                )
            });

            let is_nullable = !col
                .options
                .iter()
                .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull));

            Column {
                name: col.name.to_string(),
                data_type,
                is_primary_key,
                is_nullable,
            }
        })
        .collect()
}
