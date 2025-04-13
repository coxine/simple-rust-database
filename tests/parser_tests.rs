mod common;

use simple_rust_database::parser;
use sqlparser::ast::Statement;

#[test]
fn test_parse_create_table() {
    let sql = "CREATE TABLE users (
        id INT PRIMARY KEY,
        username VARCHAR(50) NOT NULL,
        email VARCHAR(100)
    );";

    let result = parser::parse_sql(sql);
    assert!(result.is_ok(), "SQL 解析失败: {:?}", result.err());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 1, "应该只有一条语句被解析");

    // 检查是否是 CREATE TABLE 语句
    if let Statement::CreateTable(create_table) = &statements[0] {
        // 检查表名
        assert_eq!(create_table.name.to_string(), "users", "表名应为 'users'");

        // 检查列数量
        assert_eq!(create_table.columns.len(), 3, "应该有 3 列");

        // 检查列名
        let column_names: Vec<String> = create_table
            .columns
            .iter()
            .map(|col| col.name.to_string())
            .collect();
        assert!(column_names.contains(&"id".to_string()));
        assert!(column_names.contains(&"username".to_string()));
        assert!(column_names.contains(&"email".to_string()));
    } else {
        panic!("预期解析结果为 CreateTable，实际为: {:?}", statements[0]);
    }
}

#[test]
fn test_parse_drop_table() {
    // 测试基本的 DROP TABLE
    let sql = "DROP TABLE users;";

    let result = parser::parse_sql(sql);
    assert!(result.is_ok(), "SQL 解析失败: {:?}", result.err());

    let statements = result.unwrap();
    if let Statement::Drop {
        object_type,
        names,
        if_exists,
        ..
    } = &statements[0]
    {
        assert_eq!(object_type.to_string(), "TABLE", "应该是 DROP TABLE 语句");
        assert_eq!(names.len(), 1, "应该只有一个表名");
        assert_eq!(names[0].to_string(), "users", "表名应为 'users'");
        assert!(!if_exists, "不应该有 IF EXISTS 子句");
    } else {
        panic!("预期解析结果为 Drop，实际为: {:?}", statements[0]);
    }

    // 测试带 IF EXISTS 的 DROP TABLE
    let sql = "DROP TABLE IF EXISTS users;";

    let result = parser::parse_sql(sql);
    assert!(result.is_ok());

    let statements = result.unwrap();
    if let Statement::Drop {
        object_type,
        names,
        if_exists,
        ..
    } = &statements[0]
    {
        assert_eq!(object_type.to_string(), "TABLE");
        assert_eq!(names[0].to_string(), "users");
        assert!(if_exists, "应该有 IF EXISTS 子句");
    } else {
        panic!("预期解析结果为 Drop，实际为: {:?}", statements[0]);
    }
}

#[test]
fn test_parse_multiple_statements() {
    let sql = "
        CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(100));
        DROP TABLE IF EXISTS products;
        CREATE TABLE orders (id INT PRIMARY KEY, user_id INT);
    ";

    let result = parser::parse_sql(sql);
    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3, "应该解析出三条语句");

    assert!(matches!(statements[0], Statement::CreateTable { .. }));
    assert!(matches!(statements[1], Statement::Drop { .. }));
    assert!(matches!(statements[2], Statement::CreateTable { .. }));
}

#[test]
fn test_parse_errors() {
    // 语法错误
    let sql = "CREAT TABLE users (id INT);"; // CREATE 拼写错误
    let result = parser::parse_sql(sql);
    assert!(result.is_err(), "应该报告语法错误");

    // 不完整的 SQL
    let sql = "CREATE TABLE users (";
    let result = parser::parse_sql(sql);
    assert!(result.is_err(), "应该报告语法错误");
}
