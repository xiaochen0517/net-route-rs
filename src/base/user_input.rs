use crate::base::NetRouteError;

pub fn user_check(message: &str) -> Result<(), NetRouteError> {
    if message.is_empty() {
        return Err(NetRouteError::new("输入不能为空".to_string()));
    }
    println!("{}[N]取消；[Y]确认", message);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| NetRouteError::new("读取输入失败".to_string()))?;
    let input = input.trim().to_lowercase();
    if input != "y" && input != "yes" {
        return Err(NetRouteError::new("用户取消操作".to_string()));
    }
    println!("用户确认操作");
    Ok(())
}

pub fn user_select_num(message: &str, min: usize, max: usize) -> Result<usize, NetRouteError> {
    if min > max {
        return Err(NetRouteError::new("最小值不能大于最大值".to_string()));
    }

    loop {
        println!(
            "{} (请输入 {} 到 {} 之间的数字，[N]取消)",
            message, min, max
        );
        let mut input = String::new();

        // 处理读取输入错误 - 打印错误信息并继续循环
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("读取输入失败: {}", e);
            continue;
        }

        let input = input.trim();

        // 检查用户是否取消操作
        if input.eq_ignore_ascii_case("n") {
            return Err(NetRouteError::new("用户取消操作".to_string()));
        }

        // 尝试解析输入为数字
        match input.parse::<usize>() {
            Ok(num) if num >= min && num <= max => {
                return Ok(num);
            }
            _ => {
                println!("输入无效，请重新输入！");
                continue;
            }
        }
    }
}
