use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    TableExists(String),
    TableNotFound(String),
    TypeUnmatch(String),
    FileError(String),
    ParseError(String),
    ExecutionError(String),
    DeserializationError(String, String),
    SerializationError(String, String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::TableExists(name) => write!(f, "表 '{}' 已存在", name),
            ExecutionError::TableNotFound(name) => write!(f, "表 '{}' 不存在", name),
            ExecutionError::FileError(msg) => write!(f, "文件操作错误: {}", msg),
            ExecutionError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            ExecutionError::ExecutionError(msg) => write!(f, "执行错误: {}", msg),
            ExecutionError::DeserializationError(name, msg) => {
                write!(f, "反序列化 '{}' 错误: {}", name, msg)
            }
            ExecutionError::SerializationError(name, msg) => {
                write!(f, "序列化表 '{}' 错误: {}", name, msg)
            }
            ExecutionError::TypeUnmatch(msg) => {
                write!(f, "类型不匹配: {}", msg)
            }
        }
    }
}

impl Error for ExecutionError {}

pub type ExecutionResult<T> = Result<T, ExecutionError>;
