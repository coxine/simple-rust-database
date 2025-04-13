use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

/// 测试前清理测试数据
#[allow(dead_code)]
pub fn setup() {
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

    // 强制同步文件系统
    thread::sleep(Duration::from_millis(50));
}

/// 测试后清理数据
#[allow(dead_code)]
pub fn teardown() {
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
                                thread::sleep(Duration::from_millis(50));
                            }
                        }
                    }
                }
            }
        }
    }
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
