use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    TableExists(String),
    TableNotFound(String),
    FileError(String),
    ParseError(String),
    ExecutionError(String),
    DeserializationError(String),
    SerializationError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::TableExists(name) => write!(f, "表 '{}' 已存在", name),
            ExecutionError::TableNotFound(name) => write!(f, "表 '{}' 不存在", name),
            ExecutionError::FileError(msg) => write!(f, "文件操作错误: {}", msg),
            ExecutionError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            ExecutionError::ExecutionError(msg) => write!(f, "执行错误: {}", msg),
            ExecutionError::DeserializationError(msg) => write!(f, "反序列化错误: {}", msg),
            ExecutionError::SerializationError(msg) => write!(f, "序列化错误: {}", msg),
        }
    }
}

impl Error for ExecutionError {}

pub type ExecutionResult<T> = Result<T, ExecutionError>;
