use sqlparser::ast::{ObjectType, Statement};
use std::io::ErrorKind;

// DROP: 删除一或多个数据表。
pub fn drop(stmt: &Statement) {
    if let Statement::Drop {
        object_type,
        if_exists,
        names,
        ..
    } = stmt
    {
        match object_type {
            ObjectType::Table => {
                for name in names {
                    let table_name = name.to_string();
                    let file_path = format!("data/{}.csv", table_name);

                    match std::fs::remove_file(&file_path) {
                        Ok(_) => println!("DROP: 成功删除表 {}", table_name),
                        Err(err) => match err.kind() {
                            ErrorKind::NotFound if *if_exists => {
                                println!("\x1b[33m警告\x1b[0m: 表 {} 不存在 (by IF EXISTS)", file_path);
                            }
                            ErrorKind::NotFound => {
                                eprintln!("删除表错误: 表 {} 不存在", file_path);
                            }
                            _ => {
                                eprintln!("删除表作物: {}", err);
                            }
                        },
                    }
                }
            }
            _ => println!("暂不支持删除类型: {:?}", object_type),
        }
    } else {
        println!("DROP操作失败: 无法解析AST");
    }
}
