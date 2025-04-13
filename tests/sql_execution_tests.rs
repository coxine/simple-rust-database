use simple_rust_database::executor;
use simple_rust_database::parser;
use std::fs;
use std::path::Path;

// 测试前清理测试数据
fn setup() {
    let data_dir = Path::new("data");
    if data_dir.exists() {
        // 确保目录存在
        for file in fs::read_dir(data_dir).expect("无法读取目录") {
            if let Ok(entry) = file {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                    if let Err(e) = fs::remove_file(&path) {
                        eprintln!("清理文件失败: {:?} - {}", path, e);
                    }
                }
            }
        }
    } else {
        // 确保目录存在
        fs::create_dir_all(data_dir).expect("无法创建数据目录");
    }

    // 确保所有测试文件都被删除
    let test_files = vec!["test_table.csv", "users.csv", "products.csv"];
    for file in test_files {
        let file_path = data_dir.join(file);
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                panic!("无法删除文件 {:?}: {}", file_path, e);
            }
        }
    }
    // 强制同步文件系统
    std::thread::sleep(std::time::Duration::from_millis(50));
}

// 测试后清理数据
fn teardown() {
    let data_dir = Path::new("data");
    if data_dir.exists() {
        for file in fs::read_dir(data_dir).expect("无法读取目录") {
            if let Ok(entry) = file {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                    // 多次尝试删除文件
                    for _ in 0..3 {
                        match fs::remove_file(&path) {
                            Ok(_) => break,
                            Err(e) => {
                                eprintln!("删除文件尝试失败: {:?} - {}", path, e);
                                std::thread::sleep(std::time::Duration::from_millis(50));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_create_table() {
    setup();

    // 测试创建表
    let sql = "CREATE TABLE test_table (
        id INT PRIMARY KEY,
        name VARCHAR(100),
        age INT,
        description VARCHAR(200) NOT NULL
    );";

    let result = parser::parse_sql(sql);
    assert!(result.is_ok(), "SQL 解析失败: {:?}", result.err());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 1, "应该只有一条语句被解析");

    executor::execute_statement(&statements[0]);

    // 验证文件是否已创建
    let file_path = Path::new("data/test_table.csv");
    assert!(file_path.exists(), "表文件未创建");

    // 验证文件内容
    let content = fs::read_to_string(file_path).expect("无法读取文件");
    let lines: Vec<&str> = content.lines().collect();

    assert_eq!(lines.len(), 3, "CSV 应该有三行");
    assert!(
        lines[0].contains("id")
            && lines[0].contains("name")
            && lines[0].contains("age")
            && lines[0].contains("description"),
        "CSV 表头应包含所有列名"
    );

    teardown();
}

#[test]
fn test_create_and_drop_table() {
    setup();

    // 创建表
    let create_sql = "CREATE TABLE users (
        id INT PRIMARY KEY,
        username VARCHAR(50) NOT NULL,
        email VARCHAR(100)
    );";

    let result = parser::parse_sql(create_sql);
    assert!(result.is_ok());
    executor::execute_statement(&result.unwrap()[0]);

    // 确认表已创建
    let file_path = Path::new("data/users.csv");
    assert!(file_path.exists(), "表未创建");

    // 删除表
    let drop_sql = "DROP TABLE users;";
    let result = parser::parse_sql(drop_sql);
    assert!(result.is_ok());
    executor::execute_statement(&result.unwrap()[0]);

    // 确认表已删除
    assert!(!file_path.exists(), "表未被删除");

    // 不需要再次调用 teardown()，因为我们已经显式删除了表
    // 保留 teardown 以防测试中途失败（这样也能清理资源）
    teardown();
}

#[test]
fn test_drop_nonexistent_table() {
    setup();

    // 删除不存在的表 (不带 IF EXISTS)
    let drop_sql = "DROP TABLE nonexistent_table;";
    let result = parser::parse_sql(drop_sql);
    assert!(result.is_ok());

    // 预期会有错误信息，但不会崩溃
    executor::execute_statement(&result.unwrap()[0]);

    // 使用 IF EXISTS 删除不存在的表
    let safe_drop_sql = "DROP TABLE IF EXISTS nonexistent_table;";
    let result = parser::parse_sql(safe_drop_sql);
    assert!(result.is_ok());
    executor::execute_statement(&result.unwrap()[0]);

    teardown();
}

#[test]
fn test_create_table_with_various_data_types() {
    setup();

    // 创建表，测试不同的数据类型和约束
    let create_sql = "CREATE TABLE products (
        id INT(10) PRIMARY KEY,
        name VARCHAR(200) NOT NULL,
        price INT NOT NULL,
        stock INT(5),
        description VARCHAR(500)
    );";

    let result = parser::parse_sql(create_sql);
    assert!(result.is_ok());
    executor::execute_statement(&result.unwrap()[0]);

    // 验证文件是否已创建及其内容
    let file_path = Path::new("data/products.csv");
    assert!(file_path.exists(), "表文件未创建");

    let content = fs::read_to_string(file_path).expect("无法读取文件");
    let lines: Vec<&str> = content.lines().collect();

    assert_eq!(lines.len(), 3, "CSV 应该有三行");

    // 验证第一行包含所有列名
    assert!(
        lines[0].contains("id")
            && lines[0].contains("name")
            && lines[0].contains("price")
            && lines[0].contains("stock")
            && lines[0].contains("description"),
        "CSV 表头不包含所有列名"
    );

    // 验证第二行包含长度信息
    let lengths: Vec<&str> = lines[1].split(',').collect();
    assert_eq!(lengths.len(), 5, "长度行应该有5列");
    assert_eq!(lengths[0], "10", "id 的长度应该是10");
    assert_eq!(lengths[1], "200", "name 的长度应该是200");

    // 验证第三行包含标志信息
    let flags: Vec<&str> = lines[2].split(',').collect();
    assert_eq!(flags.len(), 5, "标志行应该有5列");
    assert_eq!(flags[0], "2", "id 的标志应该是2 (主键)");
    assert_eq!(flags[1], "5", "name 的标志应该是5 (VARCHAR + NOT NULL)");

    teardown();
}
