use crate::base::NetRouteError;
use prettytable::Table;
use winroute::*;

struct WinRoute {
    manager: RouteManager,
}

impl WinRoute {
    pub fn new() -> Result<WinRoute, NetRouteError> {
        match RouteManager::new() {
            Ok(manager) => Ok(WinRoute { manager }),
            Err(e) => Err(NetRouteError {
                message: e.to_string(),
            }),
        }
    }

    pub fn get_routes(&self) -> Result<Vec<Route>, NetRouteError> {
        match self.manager.routes() {
            Ok(routes) => Ok(routes),
            Err(e) => Err(NetRouteError {
                message: e.to_string(),
            }),
        }
    }
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
        total_size,
        total_pages,
        current_page
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
    for route in routes.iter().skip((current_page - 1) * page_size).take(page_size) {
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

#[cfg(test)]
mod tests;
