/// 数据持久化存储模块
///
/// 提供数据库表的加载和保存功能，支持将表结构和内容序列化到磁盘文件，
/// 以及从磁盘文件反序列化表数据。
use crate::executor::error::{ExecutionError, ExecutionResult};
use crate::executor::TABLES;
use crate::utils;
use bincode::config;
use std::fs::{create_dir_all, read_dir, File};
use std::io::ErrorKind;
use std::path::Path;

/// 表文件扩展名
const FILE_EXTENSION: &str = "bin";

/// 加载所有数据库表
///
/// 从数据目录加载所有序列化的表文件，反序列化为表对象。
///
/// # Returns
///
/// * `ExecutionResult<()>` - 加载结果
pub fn load_all_tables() -> ExecutionResult<()> {
    let data_dir = Path::new("./data");
    if !data_dir.exists() {
        return Ok(());
    }
    let mut tables = TABLES
        .lock()
        .map_err(|e| ExecutionError::ExecutionError(format!("锁定TABLES失败: {}", e)))?;
    for entry in read_dir(data_dir)
        .map_err(|e| ExecutionError::FileError(format!("读取数据目录失败: {}", e)))?
    {
        let entry =
            entry.map_err(|e| ExecutionError::FileError(format!("读取目录项失败: {}", e)))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some(FILE_EXTENSION) {
            let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let mut file = File::open(&path).map_err(|e| {
                ExecutionError::FileError(format!("打开文件 {:?} 失败: {}", path, e))
            })?;

            let table =
                bincode::decode_from_std_read(&mut file, config::standard()).map_err(|e| {
                    ExecutionError::DeserializationError(file_name.to_string(), e.to_string())
                })?;
            tables.insert(file_name.to_string(), table);
        }
    }
    Ok(())
}

/// 保存所有数据库表
///
/// 将内存中的所有表序列化到数据目录的文件中。
///
/// # Returns
///
/// * `ExecutionResult<()>` - 保存结果
pub fn store_all_tables() -> ExecutionResult<()> {
    let data_dir = Path::new("./data");
    create_dir_all(data_dir)
        .map_err(|e| ExecutionError::FileError(format!("创建数据目录失败: {}", e)))?;
    let tables = TABLES
        .lock()
        .map_err(|e| ExecutionError::ExecutionError(format!("锁定TABLES失败: {}", e)))?;
    for (name, table) in tables.iter() {
        let file_path = data_dir.join(format!("{}.{}", name, FILE_EXTENSION));
        let mut file = File::create(&file_path).map_err(|e| {
            ExecutionError::FileError(format!("创建文件 {:?} 失败: {}", file_path, e))
        })?;

        bincode::encode_into_std_write(table, &mut file, config::standard())
            .map_err(|e| ExecutionError::SerializationError(name.to_string(), e.to_string()))?;
    }
    Ok(())
}

/// 移除表文件
///
/// 从磁盘上删除指定表的文件。
///
/// # Arguments
///
/// * `table_name` - 要删除的表名
///
/// # Returns
///
/// * `ExecutionResult<()>` - 删除结果
pub fn remove_table_file(table_name: &str) -> ExecutionResult<()> {
    let file_path = format!("data/{}.{}", table_name, FILE_EXTENSION);

    match std::fs::remove_file(&file_path) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                utils::log_warning(&format!(
                    "表文件 {} 不存在，可能原因：此表于本次会话中创建，尚未保存。",
                    table_name
                ));
                Ok(())
            }
            _ => Err(ExecutionError::FileError(format!(
                "删除表文件错误: {}",
                err
            ))),
        },
    }
}
