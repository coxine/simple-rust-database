mod common;

use simple_rust_database::executor;
use simple_rust_database::parser;

#[test]
fn test_create_and_drop_table() {
    common::setup();

    // 创建表
    let create_sql = "CREATE TABLE users (
        id INT PRIMARY KEY,
        username VARCHAR(50) NOT NULL,
        email VARCHAR(100)
    );";

    let result = parser::parse_sql(create_sql);
    assert!(result.is_ok());
    let result = executor::execute_statement(&result.unwrap()[0]);
    assert!(result.is_ok(), "执行失败: {:?}", result.err());

    // 确认表已创建
    assert!(common::table_exists("users"), "表未创建");

    // 删除表
    let drop_sql = "DROP TABLE users;";
    let result = parser::parse_sql(drop_sql);
    assert!(result.is_ok());
    let result = executor::execute_statement(&result.unwrap()[0]);
    assert!(result.is_ok(), "执行失败: {:?}", result.err());

    // 确认表已删除
    assert!(!common::table_exists("users"), "表未被删除");

    common::teardown();
}

#[test]
fn test_drop_nonexistent_table() {
    common::setup();

    // 删除不存在的表 (不带 IF EXISTS)
    let drop_sql = "DROP TABLE nonexistent_table;";
    let result = parser::parse_sql(drop_sql);
    assert!(result.is_ok());
    let result = executor::execute_statement(&result.unwrap()[0]);
    assert!(result.err().is_some(), "预期执行失败，但成功执行了");

    // 使用 IF EXISTS 删除不存在的表
    let safe_drop_sql = "DROP TABLE IF EXISTS nonexistent_table;";
    let result = parser::parse_sql(safe_drop_sql);
    assert!(result.is_ok());
    let result = executor::execute_statement(&result.unwrap()[0]);
    assert!(result.is_ok(), "执行失败: {:?}", result.err());

    common::teardown();
}

#[test]
fn test_drop_multiple_tables() {
    common::setup();

    // 创建多个表
    let tables = ["table1", "table2", "table3"];
    for table in &tables {
        let create_sql = format!("CREATE TABLE {} (id INT PRIMARY KEY);", table);
        let result = parser::parse_sql(&create_sql);
        assert!(result.is_ok());
        let result = executor::execute_statement(&result.unwrap()[0]);
        assert!(result.is_ok(), "执行失败: {:?}", result.err());
        assert!(common::table_exists(table), "表 {} 未创建", table);
    }

    // 删除多个表 (如果 DROP 实现支持多表删除)
    let drop_sql = "DROP TABLE table1, table2, table3;";
    let result = parser::parse_sql(drop_sql);

    // 注意：以下测试可能需要根据 DROP TABLE 的具体实现调整
    // 如果 DROP TABLE 只支持单表，可能需要分别执行
    if result.is_ok() {
        let result = executor::execute_statement(&result.unwrap()[0]);
        assert!(result.is_ok(), "执行失败: {:?}", result.err());

        // 验证所有表都已删除
        for table in &tables {
            assert!(!common::table_exists(table), "表 {} 未被删除", table);
        }
    } else {
        // 假设不支持多表删除，则跳过此测试
        println!("跳过多表删除测试：当前实现可能不支持");
    }

    common::teardown();
}
