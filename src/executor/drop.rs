use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use sqlparser::ast::{ObjectType, Statement};
use std::io::ErrorKind;

fn remove_table_file(table_name: &str, if_exists: bool) -> ExecutionResult<()> {
    let file_path = format!("data/{}.json", table_name);

    match std::fs::remove_file(&file_path) {
        Ok(_) => {
            println!("DROP: 成功删除表文件 {}", table_name);
            Ok(())
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound if if_exists => {
                eprintln!("DROP: 表文件 {} 不存在，跳过删除", table_name);
                Ok(())
            }
            ErrorKind::NotFound => Err(ExecutionError::TableNotFound(table_name.to_string())),
            _ => Err(ExecutionError::FileError(format!(
                "删除表文件错误: {}",
                err
            ))),
        },
    }
}

pub fn drop(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Drop {
        object_type,
        if_exists,
        names,
        ..
    } = stmt
    {
        match object_type {
            ObjectType::Table => {
                let mut tables = TABLES.lock().map_err(|e| {
                    ExecutionError::ExecutionError(format!("锁定TABLES失败: {}", e))
                })?;

                for name in names {
                    let table_name = name.to_string();

                    if tables.remove(&table_name).is_some() || *if_exists {
                        remove_table_file(&table_name, *if_exists)?;
                        println!("DROP: 成功删除表 {}", table_name);
                    } else {
                        return Err(ExecutionError::TableNotFound(table_name));
                    }
                }
                Ok(())
            }
            _ => {
                return Err(ExecutionError::ExecutionError(
                    "暂不支持删除类型".to_string(),
                ));
            }
        }
    } else {
        return Err(ExecutionError::ParseError("无法解析DROP语句".to_string()));
    }
}
