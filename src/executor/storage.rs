use crate::executor::error::{ExecutionError, ExecutionResult};
use crate::executor::TABLES;
use serde_json;
use std::fs::{create_dir_all, read_dir, File};
use std::io::BufReader;
use std::path::Path;

const FILE_EXTENSION: &str = "json";

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
            let file = File::open(&path).map_err(|e| {
                ExecutionError::FileError(format!("打开文件 {:?} 失败: {}", path, e))
            })?;
            let reader = BufReader::new(file);
            let table = serde_json::from_reader(reader).map_err(|e| {
                ExecutionError::DeserializationError(format!(
                    "反序列化JSON表 {} 失败: {}",
                    file_name, e
                ))
            })?;
            tables.insert(file_name.to_string(), table);
        }
    }
    Ok(())
}

pub fn store_all_tables() -> ExecutionResult<()> {
    let data_dir = Path::new("./data");
    create_dir_all(data_dir)
        .map_err(|e| ExecutionError::FileError(format!("创建数据目录失败: {}", e)))?;
    let tables = TABLES
        .lock()
        .map_err(|e| ExecutionError::ExecutionError(format!("锁定TABLES失败: {}", e)))?;
    for (name, table) in tables.iter() {
        let file_path = data_dir.join(format!("{}.{}", name, FILE_EXTENSION));
        let file = File::create(&file_path).map_err(|e| {
            ExecutionError::FileError(format!("创建文件 {:?} 失败: {}", file_path, e))
        })?;
        serde_json::to_writer_pretty(file, table).map_err(|e| {
            ExecutionError::SerializationError(format!("序列化JSON表{}失败: {}", name, e))
        })?;
    }
    Ok(())
}
