use sqlparser::ast::{CreateTable, DataType};
use std::fs;
use std::io::Write;
use std::path::Path;

/// 根据 CreateTable 语句创建 CSV 文件。CSV 文件内容共三行：
///
/// 1. 表头：各列名称，以逗号分隔
/// 2. 长度行：对于 `VARCHAR` 和 `INT` 类型，如果指定了长度则写出，否则留空
/// 3. `Flags` 行：每列对应一个 0-7 的标志，
///    - 最高位：`1` 表示列类型为 `VARCHAR`，`0` 表示为 `INT`
///    - 中间位：`1` 表示该列是主键
///    - 最低位：`1` 表示该列非空
///
pub fn create_csv_table(create_table_stmt: &CreateTable) {
    let table_name = create_table_stmt
        .name
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

    if let Err(e) = fs::create_dir_all("data") {
        println!("创建目录失败: {}", e);
        return;
    }

    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path)
    {
        Ok(mut file) => {
            let header_line = generate_header_line(create_table_stmt);
            let length_line = generate_length_line(create_table_stmt);
            let flag_line = generate_flag_line(create_table_stmt);

            if writeln!(file, "{}", header_line).is_err() {
                println!("写入表头失败");
            } else if writeln!(file, "{}", length_line).is_err() {
                println!("写入长度信息失败");
            } else if writeln!(file, "{}", flag_line).is_err() {
                println!("写入 flags 信息失败");
            } else {
                println!("CreateTable: 成功创建表 {}", table_name);
            }
        }
        Err(e) => println!("创建表失败: {}", e),
    }
}

fn generate_header_line(create_table_stmt: &CreateTable) -> String {
    let headers: Vec<String> = create_table_stmt
        .columns
        .iter()
        .map(|col| col.name.to_string())
        .collect();
    headers.join(",")
}

fn generate_length_line(create_table_stmt: &CreateTable) -> String {
    let lengths: Vec<String> = create_table_stmt
        .columns
        .iter()
        .map(|col| match &col.data_type {
            DataType::Varchar(opt) => opt.as_ref().map(|len| len.to_string()).unwrap_or_default(),
            DataType::Int(opt) => opt.as_ref().map(|len| len.to_string()).unwrap_or_default(),
            _ => "".to_string(),
        })
        .collect();
    lengths.join(",")
}

fn generate_flag_line(create_table_stmt: &CreateTable) -> String {
    let flags: Vec<String> = create_table_stmt
        .columns
        .iter()
        .map(|col| {
            let mut flag: u8 = 0;
            // 类型标志：Varchar 为 1 (最高位)，Int 为 0
            match &col.data_type {
                DataType::Varchar(_) => {
                    flag |= 1 << 2;
                }
                DataType::Int(_) => {} // 保持为 0
                _ => {}
            }
            // 中间位：是否为 primary key
            if col.options.iter().any(|opt| {
                matches!(
                    opt.option,
                    sqlparser::ast::ColumnOption::Unique {
                        is_primary: true,
                        ..
                    }
                )
            }) {
                flag |= 1 << 1;
            }
            // 最低位：是否为 not null
            if col
                .options
                .iter()
                .any(|opt| matches!(opt.option, sqlparser::ast::ColumnOption::NotNull))
            {
                flag |= 1;
            }
            flag.to_string()
        })
        .collect();
    flags.join(",")
}
