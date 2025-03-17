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

/// 将 IP 地址列表转换为字符串
pub fn show_interface_list() {
    let interface = Interface::new();
    let adapters = interface
        .get_interfaces()
        .map_err(|e| {
            eprintln!("Get interface list failed, error msg: {}", e.message);
        })
        .unwrap();

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
        let if_type = match adapter.if_type() {
            ipconfig::IfType::Other => "其他",
            ipconfig::IfType::EthernetCsmacd => "以太网",
            ipconfig::IfType::Iso88025Tokenring => "令牌环",
            ipconfig::IfType::Ppp => "点对点协议",
            ipconfig::IfType::SoftwareLoopback => "软件回环",
            ipconfig::IfType::Atm => "ATM",
            ipconfig::IfType::Ieee80211 => "无线局域网",
            ipconfig::IfType::Tunnel => "隧道",
            ipconfig::IfType::Ieee1394 => "IEEE 1394",
            ipconfig::IfType::Unsupported => "不支持",
            _ => "未知",
        };
        let index = adapter.ipv6_if_index();
        let name = adapter.adapter_name();
        let description = adapter.description();
        let ip_address = parse_address_list_to_string(adapter.ip_addresses());
        let mac_address = adapter.physical_address();
        let mac_address_str = mac_address
            // 将 u8 数组转换为十六进制字符串，中间用冒号分隔
            .map(|mac| {
                mac.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join(":")
            })
            .unwrap_or_else(|| "N/A".to_string());
        let gateway = parse_address_list_to_string(adapter.gateways());
        table.add_row(row![
            if_type,
            index,
            description,
            name,
            ip_address,
            mac_address_str,
            gateway
        ]);
    }
    table.printstd();
}

#[cfg(test)]
mod tests;
