use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// DEBUG 模式
    #[arg(short, long, default_value_t = 0)]
    pub debug: u8,

    /// 命令行参数
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 获取路由管理
    Route {
        /// 获取路由列表
        #[command(subcommand)]
        action: RouteActions,
    },
    /// 获取网络接口
    Interface {
        /// 获取网络接口列表
        #[command(subcommand)]
        action: InterfaceActions,
    },
}

/// 路由相关指令
#[derive(Subcommand)]
pub enum RouteActions {
    /// 展示路由列表
    List {
        /// 每页展示数量
        #[arg(long, default_value_t = 10, value_parser = less_than_one_error)]
        page_size: usize,

        /// 当前页码
        #[arg(long, default_value_t = 1, value_parser = less_than_one_error)]
        page: usize,
    },
}

/// 网络接口相关指令
#[derive(Subcommand)]
pub enum InterfaceActions {
    /// 展示网络接口列表
    List {},
}

/// 检查输入的内容是否小于 1，如果小于 1 则返回错误
///
/// # Arguments
///
/// * `s` - 输入的字符串
///
fn less_than_one_error(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<usize>()
        .map_err(|_| String::from("Invalid page size"))?;

    if value < 1 {
        return Err(String::from("Page size must be at least 1"));
    }

    Ok(value)
}
