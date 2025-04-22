use crate::executor::error::{ExecutionError, ExecutionResult};
use crate::executor::TABLES;
use bincode::config;
use std::fs::{create_dir_all, read_dir, File};
use std::io::ErrorKind;
use std::path::Path;

const FILE_EXTENSION: &str = "bin";

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

pub fn remove_table_file(table_name: &str, if_exists: bool) -> ExecutionResult<()> {
    let file_path = format!("data/{}.{}", table_name, FILE_EXTENSION);

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
