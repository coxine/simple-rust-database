// 公开相关模块供测试使用
pub mod executor;
pub mod highlighter;
pub mod parser;
pub mod repl;

pub fn execute_sql(sql_statement: &str) -> bool {
    match parser::parse_sql(sql_statement) {
        Ok(statements) => {
            for statement in statements {
                let execute_result = executor::execute_statement(&statement);
                if let Err(e) = execute_result {
                    eprintln!("执行失败: {}", e);
                    return false;
                }
            }
            true
        }
        Err(e) => {
            eprintln!("SQL 执行错误: {}", e);
            false
        }
    }
}
