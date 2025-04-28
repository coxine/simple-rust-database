// 公开相关模块供测试使用
pub mod executor;
pub mod parser;
pub mod repl;
pub mod utils;

pub fn execute_sql(sql_statement: &str) -> bool {
    match parser::parse_sql(sql_statement) {
        Ok(statements) => {
            for statement in statements {
                let execute_result = executor::execute_statement(&statement);
                if let Err(e) = execute_result {
                    utils::log_error(e.to_string());
                    return false;
                }
            }
            true
        }
        Err(e) => {
            utils::log_error(e.to_string());
            false
        }
    }
}
