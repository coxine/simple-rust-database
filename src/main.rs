use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use simple_rust_database::executor;
use simple_rust_database::parser;

fn main() {
    // 创建命令行编辑器
    let mut rl = DefaultEditor::new().unwrap();

    // 打印欢迎信息
    println!("Simple Rust Database v0.1");
    println!("输入SQL语句，以分号结束。输入'exit'退出。");

    // 读取用户输入的循环
    loop {
        match rl.readline("db> ") {
            Ok(line) => {
                // 将输入添加到历史记录
                rl.add_history_entry(line.as_str()).unwrap();

                // 检查是否退出
                if line.trim().eq_ignore_ascii_case("exit") {
                    break;
                }

                // 解析SQL语句
                match parser::parse_sql(&line) {
                    Ok(statements) => {
                        // 如果没有语句，继续下一轮
                        if statements.is_empty() {
                            continue;
                        }

                        // 执行每条SQL语句
                        for stmt in statements {
                            match executor::execute_statement(&stmt) {
                                Ok(_) => (), // 成功执行，无需额外操作
                                Err(e) => println!("执行错误: {}", e),
                            }
                        }
                    }
                    Err(e) => {
                        println!("解析错误: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("按Ctrl-D退出");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("退出");
                break;
            }
            Err(err) => {
                println!("错误: {:?}", err);
                break;
            }
        }
    }
}
