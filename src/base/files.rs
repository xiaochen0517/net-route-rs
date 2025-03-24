use crate::base::NetRouteError;
use std::fs;
use std::path::Path;

/// 读取文件内容
///
/// # Arguments
///
/// * `file_path` - 文件路径
pub fn read_file_content(file_path: &str) -> Result<String, NetRouteError> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(NetRouteError::new(format!("File not found: {}", file_path)));
    }
    let content = fs::read_to_string(path).map_err(|e| NetRouteError::new(e.to_string()))?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    /// 前置初始化，创建文件
    fn setup() {
        let _ = fs::write("test.txt", "Hello, world!\n");
    }

    /// 后置清理，删除文件
    fn teardown() {
        let _ = fs::remove_file("test.txt");
    }

    #[test_case(true, "test.txt"; "测试获取文件内容正例")]
    #[test_case(false, "none.txt"; "测试获取文件内容反例")]
    fn test_read_file_content(condition: bool, file_path: &str) {
        // 前置初始化
        setup();
        let result = read_file_content(file_path);
        assert_eq!(condition, result.is_ok());
        if condition {
            assert_eq!(result.unwrap(), "Hello, world!\n");
        }
        // 后置清理
        teardown();
    }
}
