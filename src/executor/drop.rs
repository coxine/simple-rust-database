use crate::executor::storage::remove_table_file;
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use crate::utils;
use sqlparser::ast::{ObjectType, Statement};

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
                        utils::log_info(format!("DROP: 成功删除表 {}", table_name));
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
        return Err(ExecutionError::ParseError("无法解析 DROP 语句".to_string()));
    }
}
