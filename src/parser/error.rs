/// 解析器错误处理模块
///
/// 定义了 SQL 解析过程中可能出现的错误类型和结果类型。
use std::error::Error;
use std::fmt;

/// SQL 解析器错误枚举
///
/// 包含解析 SQL 语句时可能遇到的各种错误。
#[derive(Debug)]
pub enum ParserError {
    /// SQL 解析错误，包含错误消息
    SqlParseError(String),
}

impl fmt::Display for ParserError {
    /// 为 ParserError 实现字符串表示
    ///
    /// 将错误格式化为友好的错误消息。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::SqlParseError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ParserError {}

/// 解析结果类型
///
/// 表示解析操作的结果，成功时返回泛型参数 T，失败时返回 ParserError。
pub type ParserResult<T> = Result<T, ParserError>;
