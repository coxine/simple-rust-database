/// SQL 解析器模块
///
/// 用于解析 SQL 语句并转换为内部表示形式，供执行器执行。
/// 使用外部 sqlparser 库完成基本的 SQL 语法解析工作。
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub mod error;

pub use error::{ParserError, ParserResult};

/// 解析 SQL 语句
///
/// 将 SQL 字符串解析为语句向量，供后续执行。使用通用 SQL 方言。
///
/// # Arguments
///
/// * `sql` - 要解析的 SQL 字符串
///
/// # Returns
///
/// 成功时返回语句向量，失败时返回解析错误
pub fn parse_sql(sql: &str) -> ParserResult<Vec<sqlparser::ast::Statement>> {
    let dialect = GenericDialect {};
    match Parser::parse_sql(&dialect, sql) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(ParserError::SqlParseError(e.to_string())),
    }
}
