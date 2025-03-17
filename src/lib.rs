#[macro_use]
extern crate prettytable;
mod base;
mod command;
mod interface;
mod route;

use crate::command::{Cli, Commands, InterfaceActions, RouteActions};
use clap::Parser;

pub fn run() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(command) => match command {
            Commands::Route { action } => match action {
                RouteActions::List { page, page_size } => {
                    route::show_route_list(*page_size, *page);
                }
            },
            Commands::Interface { action } => match action {
                InterfaceActions::List {} => {
                    interface::show_interface_list();
                }
            },
        },
        None => {}
    }
}
