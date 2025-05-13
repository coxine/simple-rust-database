/// 表删除操作模块
///
/// 实现 DROP TABLE 语句的解析和执行逻辑，负责删除数据库表。
use crate::executor::storage::remove_table_file;
use crate::executor::{ExecutionError, ExecutionResult, TABLES};
use crate::utils;
use sqlparser::ast::{ObjectType, Statement};

/// 执行表删除操作
///
/// 解析 DROP 语句，验证表是否存在，然后删除表及其关联文件。
///
/// # Arguments
///
/// * `stmt` - SQL 语句对象，预期为 DROP 语句
///
/// # Returns
///
/// * `ExecutionResult<()>` - 删除表的结果，成功或失败
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
                        remove_table_file(&table_name)?;
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
