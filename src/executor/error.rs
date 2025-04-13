use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    TableExists(String),
    TableNotFound(String),
    PathNotFound(String),
    FileError(String),
    ParseError(String),
    ExecutionError(String),
    WriteError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::TableExists(name) => write!(f, "表 '{}' 已存在", name),
            ExecutionError::TableNotFound(name) => write!(f, "表 '{}' 不存在", name),
            ExecutionError::PathNotFound(path) => write!(f, "路径 '{}' 不存在", path),
            ExecutionError::FileError(msg) => write!(f, "文件操作错误: {}", msg),
            ExecutionError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            ExecutionError::ExecutionError(msg) => write!(f, "执行错误: {}", msg),
            ExecutionError::WriteError(msg) => write!(f, "写入错误: {}", msg),
        }
    }
}

impl Error for ExecutionError {}

pub type ExecutionResult<T> = Result<T, ExecutionError>;