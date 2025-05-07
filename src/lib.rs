use std::sync::atomic::Ordering;

use utils::IS_INFO_OUTPUT;

pub mod executor;
pub mod model;
pub mod parser;
pub mod repl;
pub mod utils;

pub fn execute_sql(sql_statement: &str) -> bool {
    IS_INFO_OUTPUT.store(false, Ordering::Relaxed);

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
