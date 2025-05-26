use std::fs;

/// Execute SQL and capture output using subprocess approach
/// This is more reliable for integration testing as it isolates each test
fn execute_sql_with_output_capture(input_file_path: &str) -> String {
    use std::process::{Command, Stdio};

    let output = Command::new("target/debug/simple_db")
        .arg(input_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute simple-rust-database");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() {
        println!("Stderr:\n{}", stderr);
    }

    stdout
}

/// Run a single test case
fn run_test_case(test_num: &str) -> Result<(), String> {
    let input_path = format!("tests/{}/input.txt", test_num);
    let output_path = format!("tests/{}/output.txt", test_num);

    // Check if input file exists
    if !std::path::Path::new(&input_path).exists() {
        return Err(format!("Input file {} does not exist", input_path));
    }

    // Check if output file exists
    if !std::path::Path::new(&output_path).exists() {
        println!(
            "Warning: Output file {} does not exist, skipping comparison",
            output_path
        );
        // Just execute the SQL to check for errors
        let actual_output = execute_sql_with_output_capture(&input_path);

        // Check if there are any errors
        if actual_output.contains("Error:") || actual_output.contains("error") {
            return Err(format!("SQL execution failed: {}", actual_output));
        }

        println!(
            "✓ Test case {} executed successfully (no output file to compare)",
            test_num
        );
        return Ok(());
    }

    // Read expected output
    let expected_output = fs::read_to_string(&output_path)
        .map_err(|e| format!("Failed to read output file {}: {}", output_path, e))?;

    // Execute SQL and get actual output
    let actual_output = execute_sql_with_output_capture(&input_path);

    // Normalize outputs (trim whitespace, normalize line endings)
    let expected_normalized = normalize_output(&expected_output);
    let actual_normalized = normalize_output(&actual_output);

    if expected_normalized == actual_normalized {
        println!("✓ Test case {} passed", test_num);
        Ok(())
    } else {
        println!("✗ Test case {} failed", test_num);
        println!("Expected output:");
        println!("{}", expected_output);
        println!("Actual output:");
        println!("{}", actual_output);
        println!("Expected normalized:");
        println!("{:?}", expected_normalized);
        println!("Actual normalized:");
        println!("{:?}", actual_normalized);
        Err(format!("Output mismatch for test case {}", test_num))
    }
}

/// Normalize output for comparison
fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Get all test case directories
fn get_test_case_dirs() -> Result<Vec<String>, std::io::Error> {
    let test_dir = std::path::Path::new("tests");
    let mut test_cases = Vec::new();

    if test_dir.is_dir() {
        for entry in fs::read_dir(test_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Check if it's a directory and not the integration_tests.rs file
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        // Only include directories that contain input.txt
                        let input_file = path.join("input.txt");
                        if input_file.exists() {
                            test_cases.push(name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    // Sort test cases numerically
    test_cases.sort_by(|a, b| {
        let num_a: Result<i32, _> = a.parse();
        let num_b: Result<i32, _> = b.parse();

        match (num_a, num_b) {
            (Ok(a), Ok(b)) => a.cmp(&b),
            _ => a.cmp(b),
        }
    });

    Ok(test_cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_all_test_cases() {
        // 检查是否指定了特定的测试用例
        if let Ok(test_case) = std::env::var("TEST_CASE") {
            run_specific_test_case(&test_case);
            return;
        }

        // 运行所有测试用例
        run_all_tests();
    }

    /// 运行所有测试用例
    fn run_all_tests() {
        let test_cases = get_test_case_dirs().expect("Failed to get test case directories");

        if test_cases.is_empty() {
            panic!("No test cases found in tests directory");
        }

        println!("Running all {} test cases...", test_cases.len());

        let mut failed_tests = Vec::new();

        for test_case in &test_cases {
            print!("Running test case {}... ", test_case);
            match run_test_case(test_case) {
                Ok(()) => {
                    println!("✓ PASSED");
                }
                Err(e) => {
                    println!("✗ FAILED: {}", e);
                    failed_tests.push(test_case.clone());
                }
            }
        }

        if !failed_tests.is_empty() {
            panic!(
                "Failed test cases: {}. Total: {}/{} tests failed",
                failed_tests.join(", "),
                failed_tests.len(),
                test_cases.len()
            );
        }

        println!("All {} test cases passed!", test_cases.len());
    }

    /// 运行指定的单个测试用例
    fn run_specific_test_case(test_case: &str) {
        println!("Running specific test case: {}", test_case);

        match run_test_case(test_case) {
            Ok(()) => {
                println!("✓ Test case {} passed!", test_case);
            }
            Err(e) => {
                panic!("Test case {} failed: {}", test_case, e);
            }
        }
    }
}
