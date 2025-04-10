use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn parse_sql(
    sql: &str,
) -> Result<Vec<sqlparser::ast::Statement>, sqlparser::parser::ParserError> {
    let dialect = GenericDialect {};
    Parser::parse_sql(&dialect, sql)
}
