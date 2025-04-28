use simple_rust_database::{execute_sql, executor};
use std::path::Path;

#[test]
fn test_insert_operations() {
    // Setup: Make sure data directory exists
    let data_dir = Path::new("./data");
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir).expect("Failed to create data directory");
    }

    // Read the SQL test file
    let sql_path = Path::new("tests/sql/test_insert.sql");
    println!(
        "Reading SQL file from: {:?}",
        sql_path
            .canonicalize()
            .unwrap_or_else(|_| sql_path.to_path_buf())
    );

    let sql = match std::fs::read_to_string(sql_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read test_insert.sql file: {}", e);
            // Try alternative path
            let alt_path = Path::new("./tests/sql/test_insert.sql");
            println!(
                "Trying alternative path: {:?}",
                alt_path
                    .canonicalize()
                    .unwrap_or_else(|_| alt_path.to_path_buf())
            );
            std::fs::read_to_string(alt_path).expect("Failed to read test_insert.sql file")
        }
    };

    println!("Successfully read SQL file, length: {}", sql.len());

    // Split SQL into individual statements
    let statements: Vec<&str> = sql
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    println!("Found {} SQL statements", statements.len());

    // Make sure we have all expected statements
    if statements.len() < 20 {
        panic!(
            "Expected at least 20 SQL statements, but found {}",
            statements.len()
        );
    }

    // Group statements by test phases
    let create_tables = &statements[0..2];
    let normal_inserts_single_with_columns = &statements[2..6];
    let normal_inserts_multiple_with_columns = &statements[6..8];
    let normal_inserts_single_without_columns = &statements[8..10];
    let normal_inserts_multiple_without_columns = &statements[10..12];
    let error_inserts_primary_key_duplicate = &statements[12..13];
    let error_inserts_not_null_constraint = &statements[13..14];
    let error_inserts_int_length_exceeded = &statements[14..15];
    let error_inserts_primary_key_conflict = &statements[15..16];
    let error_inserts_varchar_length_exceeded = &statements[16..17];
    let error_inserts_column_count_mismatch = &statements[17..18];
    let error_inserts_type_mismatch = &statements[18..19];
    let drop_tables = &statements[19..21];

    // Clean up any existing tables
    println!("Cleaning up existing tables");
    for stmt in drop_tables {
        // Ignore errors when dropping tables - they might not exist
        let result = execute_sql(stmt);
        println!("Drop table result: {}", result);
        // Don't assert here as tables might not exist
    }

    // Make sure tables are gone before we start
    {
        let tables = executor::TABLES.lock().unwrap();
        assert!(
            !tables.contains_key("users"),
            "Users table should be dropped before test"
        );
        assert!(
            !tables.contains_key("products"),
            "Products table should be dropped before test"
        );
    } // Release lock here

    // 1. Create tables
    println!("Creating tables");
    for stmt in create_tables {
        println!("Executing: {}", stmt);
        assert!(execute_sql(stmt), "Failed to create table: {}", stmt);
    }

    // 2. Test normal inserts (single record with columns)
    println!("Testing single record inserts with columns");
    for stmt in normal_inserts_single_with_columns {
        println!("Executing: {}", stmt);
        assert!(execute_sql(stmt), "Failed to insert: {}", stmt);
    }

    // 3. Test normal inserts (multiple records with columns)
    println!("Testing multiple record inserts with columns");
    for stmt in normal_inserts_multiple_with_columns {
        println!("Executing: {}", stmt);
        assert!(
            execute_sql(stmt),
            "Failed to insert multiple records: {}",
            stmt
        );
    }

    // 4. Test normal inserts (single record without columns)
    println!("Testing single record inserts without columns");
    for stmt in normal_inserts_single_without_columns {
        println!("Executing: {}", stmt);
        assert!(
            execute_sql(stmt),
            "Failed to insert without columns: {}",
            stmt
        );
    }

    // 5. Test normal inserts (multiple records without columns)
    println!("Testing multiple record inserts without columns");
    for stmt in normal_inserts_multiple_without_columns {
        println!("Executing: {}", stmt);
        assert!(
            execute_sql(stmt),
            "Failed to insert multiple records without columns: {}",
            stmt
        );
    }

    // 6. Test error cases
    // 6.1 Primary key duplicate
    println!("Testing primary key duplicate error");
    assert!(
        !execute_sql(error_inserts_primary_key_duplicate[0]),
        "Should fail on duplicate primary key"
    );

    // 6.2 Not null constraint
    println!("Testing NOT NULL constraint error");
    assert!(
        !execute_sql(error_inserts_not_null_constraint[0]),
        "Should fail on NULL in NOT NULL column"
    );

    // 6.3 Int length exceeded
    println!("Testing INT length exceeded error");
    assert!(
        !execute_sql(error_inserts_int_length_exceeded[0]),
        "Should fail on INT length exceeded"
    );

    // 6.4 Primary key conflict
    println!("Testing primary key conflict error");
    assert!(
        !execute_sql(error_inserts_primary_key_conflict[0]),
        "Should fail on primary key conflict"
    );

    // 6.5 Varchar length exceeded
    println!("Testing VARCHAR length exceeded error");
    assert!(
        !execute_sql(error_inserts_varchar_length_exceeded[0]),
        "Should fail on VARCHAR length exceeded"
    );

    // 6.6 Column count mismatch
    println!("Testing column count mismatch error");
    assert!(
        !execute_sql(error_inserts_column_count_mismatch[0]),
        "Should fail on column count mismatch"
    );

    // 6.7 Type mismatch
    println!("Testing type mismatch error");
    assert!(
        !execute_sql(error_inserts_type_mismatch[0]),
        "Should fail on type mismatch"
    );

    // Verify data by checking the tables contain the expected number of rows
    println!("Verifying data");
    {
        let tables = match executor::TABLES.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                panic!("Failed to acquire lock on TABLES. This may indicate a deadlock.");
            }
        };

        let users = tables.get("users").expect("Users table should exist");
        let products = tables.get("products").expect("Products table should exist");

        // We expect 7 successful inserts to users table (2 single with columns, 2 multiple with columns,
        // 1 single without columns, 2 multiple without columns)
        assert_eq!(users.data.len(), 7, "Users table should have 7 rows");

        // We expect 7 successful inserts to products table (2 single with columns, 2 multiple with columns,
        // 1 single without columns, 2 multiple without columns)
        assert_eq!(products.data.len(), 7, "Products table should have 7 rows");
    } // Release lock here

    // Clean up
    println!("Cleaning up");
    for stmt in drop_tables {
        println!("Executing: {}", stmt);
        // Don't assert here, as we don't care if the drop failed because the table doesn't exist
        let result = execute_sql(stmt);
        println!("Drop table result: {}", result);
    }

    // Verify tables were dropped
    println!("Verifying tables were dropped");
    {
        let tables = match executor::TABLES.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                panic!("Failed to acquire lock on TABLES. This may indicate a deadlock.");
            }
        };

        assert!(
            !tables.contains_key("users"),
            "Users table should be dropped"
        );
        assert!(
            !tables.contains_key("products"),
            "Products table should be dropped"
        );
    } // Release lock here

    println!("Test completed successfully");
}
