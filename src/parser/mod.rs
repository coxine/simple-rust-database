use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub mod error;

pub use error::{ParserError, ParserResult};

pub fn parse_sql(sql: &str) -> ParserResult<Vec<sqlparser::ast::Statement>> {
    let dialect = GenericDialect {};
    match Parser::parse_sql(&dialect, sql) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(ParserError::SqlParseError(e.to_string())),
    }
}
