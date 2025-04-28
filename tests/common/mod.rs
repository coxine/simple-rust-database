use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

/// 测试前清理测试数据
#[allow(dead_code)]
pub fn setup() {
    if let Err(e) = try_setup() {
        eprintln!("设置测试环境失败: {}", e);
    }
}

/// 尝试设置测试环境，使用问号运算符简化错误处理
#[allow(dead_code)]
fn try_setup() -> std::io::Result<()> {
    let data_dir = Path::new("data");
    if data_dir.exists() {
        // 清理已存在的CSV文件
        for file in fs::read_dir(data_dir)? {
            let entry = file?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                fs::remove_file(&path)?;
            }
        }
    } else {
        // 创建数据目录
        fs::create_dir_all(data_dir)?;
    }

    // 强制同步文件系统
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

/// 测试后清理数据
#[allow(dead_code)]
pub fn teardown() {
    if let Err(e) = try_teardown() {
        eprintln!("清理数据失败: {}", e);
    }
}

/// 尝试清理数据，使用问号运算符简化错误处理
#[allow(dead_code)]
fn try_teardown() -> std::io::Result<()> {
    let data_dir = Path::new("data");
    if data_dir.exists() {
        for file in fs::read_dir(data_dir)? {
            let entry = file?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                // 多次尝试删除文件
                let mut success = false;
                for _ in 0..3 {
                    match fs::remove_file(&path) {
                        Ok(_) => {
                            success = true;
                            break;
                        }
                        Err(e) => {
                            eprintln!("删除文件尝试失败: {:?} - {}", path, e);
                            thread::sleep(Duration::from_millis(50));
                        }
                    }
                }
                if !success {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("无法删除文件: {:?}", path),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// 获取指定表名的 CSV 文件路径
#[allow(dead_code)]
pub fn get_table_path(table_name: &str) -> std::path::PathBuf {
    Path::new("data").join(format!("{}.csv", table_name))
}

/// 检查表是否存在
#[allow(dead_code)]
pub fn table_exists(table_name: &str) -> bool {
    get_table_path(table_name).exists()
}
