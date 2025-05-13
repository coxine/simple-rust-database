/// 执行器错误处理模块
///
/// 定义了在执行 SQL 语句过程中可能出现的各种错误类型。
use std::error::Error;
use std::fmt;

/// SQL 执行错误枚举
///
/// 包含在执行 SQL 语句时可能遇到的各种错误类型。
#[derive(Debug)]
pub enum ExecutionError {
    /// 表已存在错误
    TableExists(String),
    /// 表不存在错误
    TableNotFound(String),
    /// 类型不匹配错误
    TypeUnmatch(String),
    /// 文件操作错误
    FileError(String),
    /// SQL 语句解析错误
    ParseError(String),
    /// 通用执行错误
    ExecutionError(String),
    /// 数据反序列化错误
    DeserializationError(String, String),
    /// 数据序列化错误
    SerializationError(String, String),
    /// 主键冲突错误
    PrimaryKeyConflictError(String),
}

impl fmt::Display for ExecutionError {
    /// 为执行错误实现字符串表示
    ///
    /// 将不同类型的错误格式化为友好的错误消息。
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
            ExecutionError::PrimaryKeyConflictError(msg) => {
                write!(f, "主键冲突: {}", msg)
            }
        }
    }
}

impl Error for ExecutionError {}

/// 执行结果类型
///
/// 表示执行操作的结果，成功时返回泛型参数 T，失败时返回 ExecutionError。
pub type ExecutionResult<T> = Result<T, ExecutionError>;
