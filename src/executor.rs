use sqlparser::ast::Statement;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn execute_statement(stmt: &Statement) {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table(stmt),
        Statement::Drop { .. } => drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete(stmt),
        Statement::Update { .. } => update(stmt),
        _ => println!("未识别的命令"),
    }
}

fn query(stmt: &Statement) {
    println!("Query: {:?}", stmt);
}

fn create_table(stmt: &Statement) {
    if let Statement::CreateTable(create_table_stmt) = stmt {
        let name = &create_table_stmt.name;
        let table_name = name
            .0
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<String>>()
            .join("_");
        let file_path = format!("data/{}.csv", table_name);
        let path = Path::new(&file_path);

        if path.exists() {
            println!("创建表失败: 表 {} 已存在", table_name);
            return;
        }

        // 尝试创建目录（如果 data 目录不存在）
        if let Err(e) = fs::create_dir_all("data") {
            println!("创建目录失败: {}", e);
            return;
        }

        // 创建文件
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path)
        {
            Ok(mut file) => {
                // 写入 CSV 表头，默认写入空表头或者你需要的格式
                // 此处可以根据需求设定字段，比如 "id,name,age" 等
                // 目前暂写入空内容
                if let Err(e) = writeln!(file, "") {
                    println!("写入文件失败: {}", e);
                } else {
                    println!("CreateTable: 成功创建表 {}", table_name);
                }
            }
            Err(e) => println!("创建表失败: {}", e),
        }
    } else {
        println!("创建表失败: 无法解析表名");
    }
}

fn drop(stmt: &Statement) {
    println!("Drop: {:?}", stmt);
}

fn insert(stmt: &Statement) {
    println!("Insert: {:?}", stmt);
}

fn delete(stmt: &Statement) {
    println!("Delete: {:?}", stmt);
}

fn update(stmt: &Statement) {
    println!("Update: {:?}", stmt);
}
