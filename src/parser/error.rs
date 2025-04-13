use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    SqlParseError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::SqlParseError(msg) => write!(f, "SQL解析错误: {}", msg),
        }
    }
}

impl Error for ParserError {}

pub type ParserResult<T> = Result<T, ParserError>;