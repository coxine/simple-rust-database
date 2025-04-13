use crate::executor::{ExecutionError, ExecutionResult};
use sqlparser::ast::{ObjectType, Statement};
use std::io::ErrorKind;

// DROP: 删除一或多个数据表。
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
                for name in names {
                    let table_name = name.to_string();
                    let file_path = format!("data/{}.csv", table_name);

                    match std::fs::remove_file(&file_path) {
                        Ok(_) => println!("DROP: 成功删除表 {}", table_name),
                        Err(err) => match err.kind() {
                            ErrorKind::NotFound if *if_exists => {
                                eprintln!("DROP: 表 {} 不存在，跳过删除", table_name);
                                return Ok(());
                            }

                            ErrorKind::NotFound => {
                                return Err(ExecutionError::TableNotFound(table_name))
                            }
                            _ => {
                                return Err(ExecutionError::FileError(format!(
                                    "删除表错误: {}",
                                    err
                                )));
                            }
                        },
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
