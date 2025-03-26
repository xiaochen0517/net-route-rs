use crate::base::{NetRouteError, files, user_input};
use crate::interface::{AdapterInfo, Interface};
use crate::route::config::RouteConfigData;
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

    pub fn search_route_by_ip(
        &self,
        dest: &IpAddr,
        prefix: &u8,
        if_index: Option<&u32>,
        gateway: Option<&IpAddr>,
    ) -> Result<Vec<Route>, NetRouteError> {
        Ok(self
            .get_routes()?
            .into_iter()
            .filter(|route| {
                // 根据提供的参数过滤路由
                (route.destination == *dest)
                    && (route.prefix == *prefix)
                    && (if_index.is_none() || route.ifindex == Some(*if_index.unwrap()))
                    && (gateway.is_none() || route.gateway == *gateway.unwrap())
            })
            .collect::<Vec<Route>>())
    }

    pub fn search_route_by_ip_vec(
        &self,
        ip_vec: Vec<IpAddr>,
        prefix: &u8,
        if_index: &Option<u32>,
        gateway: Option<&IpAddr>,
    ) -> Result<Vec<Route>, NetRouteError> {
        Ok(self
            .get_routes()?
            .into_iter()
            .filter(|route| {
                // 根据提供的参数过滤路由
                (ip_vec.contains(&route.destination))
                    && (route.prefix == *prefix)
                    && (if_index.is_none() || route.ifindex == Some(if_index.unwrap()))
                    && (gateway.is_none() || route.gateway == *gateway.unwrap())
            })
            .collect::<Vec<Route>>())
    }

    pub fn add_route(&self, route: &Route) -> Result<(), NetRouteError> {
        self.manager
            .add_route(route)
            .map_err(|err| NetRouteError::new(format!("添加路由错误: {}", err)))?;
        Ok(())
    }

    pub fn add_ip_route(
        &self,
        destination: IpAddr,
        prefix: &u8,
        if_index: &u32,
        gateway: IpAddr,
        metric: &u32,
    ) -> Result<Route, NetRouteError> {
        // 创建路由
        let mut route = Route::new(destination, *prefix);
        route = route.ifindex(*if_index);
        route = route.gateway(gateway);
        route = route.metric(*metric);
        // 查询路由表
        let search_route_vec =
            self.search_route_by_ip(&destination, prefix, Some(if_index), None)?;
        if !search_route_vec.is_empty() {
            println!("路由表中已存在匹配的路由！");
            show_route_table(&search_route_vec);
            return Err(NetRouteError::new("路由已存在".to_string()));
        }
        self.manager
            .add_route(&route)
            .map_err(|err| NetRouteError::new(format!("添加路由错误: {}", err)))?;
        Ok(route)
    }

    pub fn remove_route(&self, route: &Route) -> Result<(), NetRouteError> {
        self.manager
            .delete_route(route)
            .map_err(|err| NetRouteError::new(format!("删除路由错误: {}", err)))?;
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
            "-n", "2", // 发送4个数据包
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

/// 展示路由列表表格
///
/// # Arguments
///
/// * `route_vec` - 路由列表
///
pub fn show_route_table(route_vec: &Vec<Route>) {
    // 实现表格展示路由列表
    let mut table = Table::new();
    table.add_row(row![
        "序号",
        "目标地址",
        "prefix",
        "网关地址",
        "目标网卡",
        "metric",
        "LUID",
        "协议版本"
    ]);
    for (idx, route) in route_vec.iter().enumerate() {
        table.add_row(row![
            idx, // 添加从0开始的序号
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

    let route_vec = routes
        .into_iter()
        .skip((current_page - 1) * page_size)
        .take(page_size)
        .collect::<Vec<Route>>();
    // 打印路由列表
    show_route_table(&route_vec);
    Ok(())
}

pub fn get_adapter_by_if_index(if_index: &u32) -> Result<AdapterInfo, NetRouteError> {
    let interface = Interface::new();
    let adapter = interface.get_interface_by_index(if_index)?;
    Ok(adapter)
}

pub fn get_gateway_ip_by_if_index(if_index: &u32) -> Result<IpAddr, NetRouteError> {
    let adapter = get_adapter_by_if_index(if_index)?;
    let ipv4_gateway = Interface::get_ipv4_gateway(&adapter)?;
    Ok(ipv4_gateway)
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
    let ipv4_gateway = get_gateway_ip_by_if_index(if_index)?;
    // 解析目标地址
    let dest_ip: IpAddr = destination.parse().map_err(|_| {
        NetRouteError::new(format!("Invalid destination IP address: {}", destination))
    })?;
    // 检查目标地址和网卡是否可达
    if !*no_check {
        let adapter = get_adapter_by_if_index(if_index)?;
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
    let route = win_route.add_ip_route(dest_ip, prefix, if_index, gateway, metric)?;
    // 显示路由表
    println!("路由添加成功！");
    show_route_table(&vec![route]);
    Ok(())
}

/// 解析域名的IP地址列表
///
/// # Arguments
///
/// * `domain` - 域名
fn parse_domain(domain: &String) -> Result<Vec<IpAddr>, NetRouteError> {
    // 解析域名的IP地址列表
    let ip_list = dns_lookup::lookup_host(domain)
        .map_err(|_| NetRouteError::new(format!("Invalid domain name: {}", domain)))?;
    Ok(ip_list
        .into_iter()
        .filter(|ip| ip.is_ipv4())
        .collect::<Vec<IpAddr>>())
}

/// 显示域名的IP地址列表
///
/// # Arguments
///
/// * `domain` - 域名
pub fn show_domain_ips_info(domain: &String) -> Result<(), NetRouteError> {
    let ip_list = parse_domain(domain)?;
    let mut table = Table::new();
    table.add_row(row!["序号", "IP地址"]);
    for (idx, ip) in ip_list.iter().enumerate() {
        table.add_row(row![idx, ip.to_string()]);
    }
    table.printstd();
    Ok(())
}

/// 添加域名路由
///
/// # Arguments
///
/// * `domain` - 域名
/// * `if_index` - 网卡索引
/// * `metric` - 路由度量值，值越小优先级越高
/// * `no_check` - 是否检查目标地址是否可达
///
pub fn add_domain_route(
    domain: &String,
    if_index: &u32,
    metric: &u32,
    no_check: &bool,
) -> Result<(), NetRouteError> {
    // 解析域名的IP地址列表
    let ip_list = parse_domain(&domain)?;
    // 获取网卡信息
    let interface = Interface::new();
    let adapter = interface.get_interface_by_index(if_index)?;
    let gateway = Interface::get_ipv4_gateway(&adapter)?;
    // 逐个检查IP地址是否可达
    if !*no_check {
        for ip in ip_list.iter() {
            ping_from_interface(&ip.to_string(), &adapter)?;
        }
    }
    // 逐个添加路由信息
    let mut added_routes = vec![];
    for dest_ip in ip_list {
        let win_route = WinRoute::new()?;
        let added_route = win_route.add_ip_route(dest_ip, &32, if_index, gateway, metric)?;
        added_routes.push(added_route);
    }
    // 显示路由表
    println!("路由添加成功！");
    show_route_table(&added_routes);
    Ok(())
}

/// 删除路由
///
/// # Arguments
///
/// * `destination` - 目标 IP 地址
/// * `prefix` - 目标 IP 子网掩码
///
pub fn remove_route(destination: &String, prefix: &u8) -> Result<(), NetRouteError> {
    // 解析目标地址
    let dest_ip: IpAddr = destination.parse().map_err(|_| {
        NetRouteError::new(format!("Invalid destination IP address: {}", destination))
    })?;
    // 查询路由表
    let win_route = WinRoute::new()?;
    let route_vec = win_route.search_route_by_ip(&dest_ip, prefix, None, None)?;
    if route_vec.is_empty() {
        println!("路由表中没有找到匹配的路由: {}/{}", dest_ip, prefix);
        return Ok(());
    } else {
        println!("匹配到的路由:");
        show_route_table(&route_vec);
    }
    // 如果匹配的路由不止一条，提示用户选择
    let mut num = 0;
    if route_vec.len() > 1 {
        num = user_input::user_select_num("请选择需要删除的路由序号", 0, route_vec.len())?;
    }
    // 如果找到匹配的路由，需要用户确认删除
    user_input::user_check("是否删除匹配的路由？")?;
    // 创建路由
    let route = match route_vec.get(num) {
        Some(route) => route,
        None => {
            return Err(NetRouteError::new(format!(
                "路由表中没有找到匹配的路由: {}",
                dest_ip
            )));
        }
    };
    win_route.remove_route(route)?;
    // 显示路由表
    println!("路由移除成功！");
    show_route_table(&vec![(*route).clone()]);
    Ok(())
}

/// 删除域名路由
///
/// # Arguments
///
/// * `domain` - 域名
/// * `if_index` - 网卡索引
///
pub fn remove_domain_route(domain: &String, if_index: &Option<u32>) -> Result<(), NetRouteError> {
    // 解析域名的IP地址列表
    let ip_list = parse_domain(&domain)?;
    // 获取路由信息
    let win_route = WinRoute::new()?;
    // 查询路由表
    let route_list = win_route.search_route_by_ip_vec(ip_list, &32, if_index, None)?;
    if route_list.is_empty() {
        println!("路由表中没有找到匹配的路由: {}", domain);
        return Ok(());
    } else {
        println!("匹配到的路由:");
        show_route_table(&route_list);
    }
    user_input::user_check("是否删除所有匹配的路由？")?;
    // 删除路由
    for route in route_list.iter() {
        win_route.remove_route(route)?;
    }
    // 显示路由表
    println!("路由移除成功！");
    show_route_table(&route_list);
    Ok(())
}

fn parse_config_to_repeat_and_add_routes(
    win_route: &WinRoute,
    route_config_data: RouteConfigData,
) -> Result<(Vec<Route>, Vec<Route>), NetRouteError> {
    let mut repeat_route_vec = vec![];
    let mut add_route_list = vec![];
    for route_config in route_config_data.routes {
        let mut add_ip_addr_list = vec![];
        // 解析域名的IP地址列表
        for domain in route_config.domains {
            // 解析域名的IP地址列表
            let parsed_ip_list = parse_domain(&domain)?;
            add_ip_addr_list.extend(parsed_ip_list.clone());
            // 查询路由表是否存在重复的路由
            repeat_route_vec.extend(win_route.search_route_by_ip_vec(
                parsed_ip_list,
                &32,
                &None,
                None,
            )?);
        }
        let ip_addr_vec = route_config
            .ips
            .iter()
            .map(|ip_str| {
                ip_str.parse::<IpAddr>().map_err(|_| {
                    NetRouteError::new(format!("Invalid destination IP address: {}", ip_str))
                })
            })
            .collect::<Result<Vec<IpAddr>, NetRouteError>>()?;
        add_ip_addr_list.extend(ip_addr_vec.clone());
        let route_vec = win_route.search_route_by_ip_vec(ip_addr_vec, &32, &None, None)?;
        repeat_route_vec.extend(route_vec);

        // 生成路由
        let if_index = route_config.ifindex;
        // 获取ifindex
        let gateway_ip = get_gateway_ip_by_if_index(&if_index)?;
        for add_ip_addr in add_ip_addr_list {
            let mut route = Route::new(add_ip_addr, 32);
            route = route.ifindex(if_index);
            route = route.gateway(gateway_ip);
            route = route.metric(0);
            add_route_list.push(route);
        }
    }
    Ok((repeat_route_vec, add_route_list))
}

/// 应用配置文件
///
/// # Arguments
///
/// * `config_path` - 配置文件路径
/// * `no_confirm` - 是否跳过确认
///
pub fn apply_config_file(
    config_path: &Option<String>,
    no_confirm: &bool,
    cancel: &bool,
) -> Result<(), NetRouteError> {
    let path = config_path
        .as_ref()
        .ok_or_else(|| NetRouteError::new("配置文件路径不能为空".to_string()))?;
    let file_content = files::read_file_content(path)?;
    let config = config::parse_config_file(&file_content)?;

    let win_route = WinRoute::new()?;
    let (repeat_route_vec, add_route_list) =
        parse_config_to_repeat_and_add_routes(&win_route, config)?;
    if !repeat_route_vec.is_empty() {
        println!("路由表中已存在匹配的路由！");
        show_route_table(&repeat_route_vec);
        if !*no_confirm {
            user_input::user_check("是否移除已存在的路由？")?;
        }
        // 移除重复的路由
        for route in repeat_route_vec.iter() {
            println!("移除路由: {}", route.destination);
            win_route.remove_route(route)?;
        }
        println!("已移除重复的路由！");
    }
    if *cancel {
        println!("已取消应用此配置文件！");
        return Ok(());
    }
    println!("需要添加的路由:");
    show_route_table(&add_route_list);
    if !*no_confirm {
        user_input::user_check("是否继续添加路由？")?;
    }
    // 添加路由
    for route in add_route_list.iter() {
        println!("添加路由: {}", route.destination);
        win_route.add_route(route)?;
    }
    println!("路由添加成功！");
    Ok(())
}

mod config;
#[cfg(test)]
mod tests;
