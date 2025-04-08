mod executor;
mod parser;

use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut sql = String::new();
        io::stdin().read_line(&mut sql).expect("读取输入失败");

        let sql = sql.trim();

        if sql.eq_ignore_ascii_case("exit") {
            break;
        }

        match parser::parse_sql(sql) {
            Ok(ast) => {
                for stmt in &ast {
                    executor::execute_statement(stmt);
                }
            }
            Err(e) => println!("解析错误: {}\n如需退出请输入`exit`", e),
        }
    }
}
