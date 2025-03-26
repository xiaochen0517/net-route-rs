#[macro_use]
extern crate prettytable;
mod base;
mod command;
mod interface;
mod route;

use crate::base::NetRouteError;
use crate::command::{Cli, Commands, InterfaceActions, NetActions, RouteActions, RouteAddActions};
use clap::Parser;

/// 程序入口主方法
///
pub fn run() -> Result<(), NetRouteError> {
    let cli = Cli::parse();

    // 处理子命令
    match &cli.command {
        Some(command) => match command {
            Commands::Route { action } => match action {
                RouteActions::List { page, page_size } => route::show_route_list(*page_size, *page),
                RouteActions::Add { action } => match action {
                    RouteAddActions::Ip {
                        destination,
                        prefix,
                        if_index,
                        gateway,
                        metric,
                        no_check,
                    } => route::add_route(destination, prefix, if_index, gateway, metric, no_check),
                    RouteAddActions::Domain {
                        domain,
                        if_index,
                        metric,
                        no_check,
                    } => route::add_domain_route(domain, if_index, metric, no_check),
                },
                RouteActions::Remove {
                    destination,
                    domain,
                    if_index,
                    prefix,
                } => {
                    if destination.is_empty() {
                        Ok(route::remove_domain_route(domain, if_index)?)
                    } else if domain.is_empty() {
                        Ok(route::remove_route(destination, prefix)?)
                    } else {
                        Err(NetRouteError::new(
                            "目标 IP 地址和域名必须有一个不为空".to_string(),
                        ))
                    }
                }
            },
            Commands::Interface { action } => match action {
                InterfaceActions::List {} => interface::show_interface_list(),
            },
            Commands::Net { action } => match action {
                NetActions::Dns { domain } => route::show_domain_ips_info(domain),
            },
            Commands::Config {
                path,
                no_confirm,
                cancel,
            } => Ok(route::apply_config_file(path, no_confirm, cancel)?),
        },
        None => {
            println!("无效的命令");
            Ok(())
        }
    }
}
