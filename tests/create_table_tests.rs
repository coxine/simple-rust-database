mod common;

use simple_rust_database::executor;
use simple_rust_database::parser;
use std::fs;

#[test]
fn test_create_table() {
    common::setup();

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

    let result = executor::execute_statement(&statements[0]);
    assert!(result.is_ok(), "执行失败: {:?}", result.err());

    // 验证文件是否已创建
    let file_path = common::get_table_path("test_table");
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

    common::teardown();
}

#[test]
fn test_create_table_with_various_data_types() {
    common::setup();

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
    let result = executor::execute_statement(&result.unwrap()[0]);
    assert!(result.is_ok(), "执行失败: {:?}", result.err());

    // 验证文件是否已创建及其内容
    let file_path = common::get_table_path("products");
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

    common::teardown();
}
