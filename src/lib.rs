pub mod executor;
pub mod parser;
pub mod repl;
pub mod utils;

pub fn execute_sql(sql_statement: &str) -> bool {
    match parser::parse_sql(sql_statement) {
        Ok(statements) => {
            for statement in statements {
                let execute_result = executor::execute_statement(&statement);
                if let Err(_) = execute_result {
                    return false;
                }
            }
            true
        }
        Err(_) => {
            println!("Error: Syntax error");
            false
        }
    }
}
