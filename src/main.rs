use simple_db::execute_sql;

/// 简易 Rust 数据库程序
///
/// 这个程序实现了一个简单的数据库系统，支持基本的 SQL 操作，
/// 包括创建表、插入数据、查询数据、更新和删除数据等功能。
/// 提供一个交互式的 REPL 环境，让用户可以直接执行 SQL 命令。
mod executor;
mod model;
mod parser;
mod repl;
mod utils;

/// 程序入口函数
///
/// 启动一个交互式的 REPL（读取-求值-打印-循环）环境，让用户可以执行 SQL 命令
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => repl::run_repl()?,
        2 => {
            let filename = args[1].clone();
            let path = std::path::Path::new(&filename);
            if path.exists() {
                let sql = std::fs::read_to_string(path)?;
                execute_sql(&sql);
            } else {
                return Err("File note found".into());
            }
        }
        _ => return Err("Invalid arguments".into()),
    }
    Ok(())
}
