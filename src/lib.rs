#[macro_use]
extern crate prettytable;
mod base;
mod command;
mod interface;
mod route;

use crate::base::NetRouteError;
use crate::command::{Cli, Commands, InterfaceActions, RouteActions, RouteAddActions};
use clap::Parser;

/// 程序入口主方法
///
pub fn run() -> Result<(), NetRouteError> {
    let cli = Cli::parse();

    // 打印调试信息
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

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
                    } => route::add_route(destination, prefix, if_index, gateway, metric),
                },
            },
            Commands::Interface { action } => match action {
                InterfaceActions::List {} => interface::show_interface_list(),
            },
        },
        None => {
            println!("无效的命令");
            Ok(())
        }
    }
}
