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

pub fn show_route_list(page_size: usize, page: usize) {
    // 获取路由列表
    let win_route = WinRoute::new()
        .map_err(|e| {
            eprintln!("Get route manager failed, error msg: {}", e.message);
        })
        .unwrap();
    let routes = win_route
        .get_routes()
        .map_err(|e| {
            eprintln!("Get route list failed, error msg: {}", e.message);
        })
        .unwrap();

    // 计算总页数
    let page = page - 1;
    let total_items = routes.len();
    let total_pages = (total_items + page_size - 1) / page_size;
    // 计算当前页码
    let current_page = if page >= total_pages {
        total_pages - 1
    } else {
        page
    };
    println!();
    println!(
        "总数: {}, 总页数: {}, 当前页: {}",
        total_items,
        total_pages,
        current_page + 1
    );

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
    for route in routes.iter().skip(page * page_size).take(page_size) {
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
}

#[cfg(test)]
mod tests;
