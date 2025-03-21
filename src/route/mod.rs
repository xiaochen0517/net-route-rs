use crate::base::NetRouteError;
use crate::interface::{AdapterInfo, Interface};
use encoding_rs::GBK;
use prettytable::Table;
use std::net::IpAddr;
use std::process::Command;
use winroute::*;

struct WinRoute {
    manager: RouteManager,
}

impl WinRoute {
    pub fn new() -> Result<WinRoute, NetRouteError> {
        match RouteManager::new() {
            Ok(manager) => Ok(WinRoute { manager }),
            Err(e) => Err(NetRouteError::new(e.to_string())),
        }
    }

    pub fn get_routes(&self) -> Result<Vec<Route>, NetRouteError> {
        match self.manager.routes() {
            Ok(routes) => Ok(routes),
            Err(e) => Err(NetRouteError::new(e.to_string())),
        }
    }

    pub fn add_ip_route(
        &self,
        destination: IpAddr,
        prefix: &u8,
        if_index: &u32,
        gateway: IpAddr,
        metric: &u32,
    ) -> Result<(), NetRouteError> {
        // 创建路由
        let mut route = Route::new(destination, *prefix);
        route = route.ifindex(*if_index);
        route = route.gateway(gateway);
        route = route.metric(*metric);
        self.manager
            .add_route(&route)
            .map_err(|err| NetRouteError::new(format!("添加路由错误: {}", err)))?;
        println!("添加路由成功: {:?}", route);
        Ok(())
    }
}

/// 使用指定网卡的IP地址进行ping测试
///
/// # Arguments
///
/// * `target_ip` - 目标IP地址
/// * `adapter_info` - 网卡信息
///
pub fn ping_from_interface(
    target_ip: &String,
    adapter_info: &AdapterInfo,
) -> Result<bool, NetRouteError> {
    // 获取网卡的IP地址作为源地址
    let source_ip = &adapter_info.ip_address;

    println!(
        "使用网卡 {} (IP: {}) 测试连接到 {}",
        adapter_info.name, source_ip, target_ip
    );

    // 在Windows上使用ping命令，通过-S参数指定源IP
    let output = Command::new("ping")
        .args(&[
            "-n", "4", // 发送4个数据包
            "-w", "1000", // 超时时间1秒
            "-S", &source_ip, // 指定源IP地址
            target_ip,  // 目标IP地址
        ])
        .output()
        .map_err(|e| NetRouteError::new(format!("网络通路测试失败: {}", e)))?;

    let output_str = GBK.decode(&output.stdout).0.into_owned();

    // 检查是否收到回复
    let success = output_str.contains("来自")
        && output_str.contains("字节=")
        && !output_str.contains("100% 丢失");

    if success {
        println!("连接测试成功，目标IP可达");
    } else {
        println!("连接测试失败，无法连接到目标IP");
        println!("详细输出: \n{}", output_str);
    }

    Ok(success)
}

/// 计算总页数和当前页码并打印信息
///
/// # Arguments
///
/// * `total_size` - 总数据量
/// * `page_size` - 每页数据量
/// * `current_page` - 当前页码，从 1 开始
///
fn parse_page_info(total_size: usize, page_size: usize, current_page: usize) -> usize {
    let total_pages = (total_size + page_size - 1) / page_size;
    // 计算当前页码
    let current_page = if current_page > total_pages {
        total_pages
    } else {
        current_page
    };
    println!(
        "总数: {}, 总页数: {}, 当前页: {}",
        total_size, total_pages, current_page
    );
    current_page
}

/// 展示路由列表
///
/// # Arguments
///
/// * `page_size` - 每页展示数量
/// * `current_page` - 当前页码，从 1 开始
///
pub fn show_route_list(page_size: usize, current_page: usize) -> Result<(), NetRouteError> {
    // 获取路由列表
    let win_route = WinRoute::new()?;
    let routes = win_route.get_routes()?;

    // 计算总页数
    let current_page = parse_page_info(routes.len(), page_size, current_page);

    // 实现表格展示路由列表
    let mut table = Table::new();
    table.add_row(row![
        "目标地址",
        "prefix",
        "网关地址",
        "目标网卡",
        "metric",
        "LUID",
        "协议版本"
    ]);
    for route in routes
        .iter()
        .skip((current_page - 1) * page_size)
        .take(page_size)
    {
        table.add_row(row![
            route.destination.to_string(),
            route.prefix.to_string(),
            route.gateway.to_string(),
            route.ifindex.map_or("NONE".to_string(), |v| v.to_string()),
            route.metric.map_or("NONE".to_string(), |v| v.to_string()),
            route.luid.map_or("NONE".to_string(), |v| v.to_string()),
            format!("IPv{}", route.version)
        ]);
    }
    table.printstd();
    Ok(())
}

/// 添加路由
///
/// # Arguments
///
/// * `destination` - 目标 IP 地址
/// * `prefix` - 目标 IP 子网掩码
/// * `if_index` - 网卡索引
/// * `gateway` - 网关 IP 地址
/// * `metric` - 路由度量值，值越小优先级越高
/// * `no_check` - 是否检查目标地址是否可达
///
pub fn add_route(
    destination: &String,
    prefix: &u8,
    if_index: &u32,
    gateway: &Option<String>,
    metric: &u32,
    no_check: &bool,
) -> Result<(), NetRouteError> {
    // 检查if_index网卡是否存在
    let interface = Interface::new();
    let adapter = interface.get_interface_by_index(if_index)?;
    let ipv4_gateway = Interface::get_ipv4_gateway(&adapter)?;
    // 解析目标地址
    let dest_ip: IpAddr = destination.parse().map_err(|_| {
        NetRouteError::new(format!("Invalid destination IP address: {}", destination))
    })?;
    // 检查目标地址和网卡是否可达
    if !*no_check {
        ping_from_interface(destination, &adapter)?;
    }
    // 解析网关地址
    let gateway: IpAddr = match gateway {
        Some(gateway) => gateway
            .parse()
            .map_err(|_| NetRouteError::new(format!("Invalid gateway IP address: {}", gateway)))?,
        None => ipv4_gateway,
    };
    let win_route = WinRoute::new()?;
    win_route.add_ip_route(dest_ip, prefix, if_index, gateway, metric)
}

#[cfg(test)]
mod tests;
