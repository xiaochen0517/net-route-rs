use crate::base::NetRouteError;
use ipconfig;
use ipconfig::Adapter;
use prettytable::Table;
use std::net::IpAddr;

pub struct Interface;

impl Interface {
    pub fn new() -> Self {
        Interface
    }

    pub fn get_interfaces(&self) -> Result<Vec<Adapter>, NetRouteError> {
        ipconfig::get_adapters().map_err(|e| NetRouteError {
            message: e.to_string(),
        })
    }
}

/// 将 IP 地址列表转换为字符串
///
/// 如果没有 IPv4 地址，则返回 "N/A"
/// 如果有 IPv4 地址，则返回以逗号分隔的字符串
///
/// # Arguments
///
/// * `addresses` - IP 地址列表
///
fn parse_address_list_to_string(addresses: &[IpAddr]) -> String {
    let ipv4_addresses = addresses
        .iter()
        .filter(|ip| ip.is_ipv4())
        .map(|ip| ip.to_string())
        .collect::<Vec<String>>();

    if ipv4_addresses.is_empty() {
        "N/A".to_string()
    } else {
        ipv4_addresses.join(", ")
    }
}

/// 将网卡类型转换为字符串
/// 
/// 如果网卡类型未知，则返回 "未知"
/// 
/// # Arguments
/// 
/// * `if_type` - 网卡类型
fn parse_if_type(if_type: ipconfig::IfType) -> String {
    match if_type {
        ipconfig::IfType::Other => String::from("其他"),
        ipconfig::IfType::EthernetCsmacd => String::from("以太网"),
        ipconfig::IfType::Iso88025Tokenring => String::from("令牌环"),
        ipconfig::IfType::Ppp => String::from("点对点协议"),
        ipconfig::IfType::SoftwareLoopback => String::from("软件回环"),
        ipconfig::IfType::Atm => String::from("ATM"),
        ipconfig::IfType::Ieee80211 => String::from("无线局域网"),
        ipconfig::IfType::Tunnel => String::from("隧道"),
        ipconfig::IfType::Ieee1394 => String::from("IEEE 1394"),
        ipconfig::IfType::Unsupported => String::from("不支持"),
        _ => String::from("未知"),
    }
}

/// 将 MAC 地址转换为字符串
/// 
/// 如果没有 MAC 地址，则返回 "N/A"
/// 
/// # Arguments
/// 
/// * `mac_address` - MAC 地址
/// 
fn parse_mac_address(mac_address: Option<&[u8]>) -> String {
    mac_address
        // 将 u8 数组转换为十六进制字符串，中间用冒号分隔
        .map(|mac| {
            mac.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join(":")
        })
        .unwrap_or_else(|| "N/A".to_string())
}

/// 将 IP 地址列表转换为字符串
/// 
pub fn show_interface_list() -> Result<(), NetRouteError> {
    let interface = Interface::new();
    let adapters = interface.get_interfaces()?;

    // 实现表格展示路由列表
    let mut table = Table::new();
    table.add_row(row![
        "网卡类型",
        "网卡Index",
        "网卡名称",
        "网卡ID",
        "IP地址",
        "MAC地址",
        "网关地址"
    ]);
    for adapter in adapters {
        table.add_row(row![
            parse_if_type(adapter.if_type()),
            adapter.ipv6_if_index(),
            adapter.description(),
            adapter.adapter_name(),
            parse_address_list_to_string(adapter.ip_addresses()),
            parse_mac_address(adapter.physical_address()),
            parse_address_list_to_string(adapter.gateways())
        ]);
    }
    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests;
