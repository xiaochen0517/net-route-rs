use crate::base::NetRouteError;
use ipconfig;
use ipconfig::IfType;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use prettytable::Table;
use std::net::IpAddr;

pub struct Interface;

pub struct AdapterInfo {
    pub name: String,
    pub index: u32,
    pub mac_address: String,
    pub ip_address: String,
    pub gateway: String,
    pub if_type: IfType,
}

impl Interface {
    pub fn new() -> Self {
        Interface
    }

    pub fn get_interfaces(&self) -> Result<Vec<AdapterInfo>, NetRouteError> {
        // 使用 network-interface 获取适配器信息，备用
        let ni_interfaces =
            NetworkInterface::show().map_err(|e| NetRouteError::new(e.to_string()))?;
        // 获取所有适配器信息
        let adapter_info_vec = ipconfig::get_adapters()
            .map_err(|e| NetRouteError::new(e.to_string()))?
            .into_iter()
            .map(|adapter| {
                let mac_address = parse_mac_address(adapter.physical_address());
                // 判断当前是否有获取不到 index 的适配器
                let mut index = adapter.ipv6_if_index();
                if index == 0 {
                    index = find_interface_index_by_mac(&ni_interfaces, &mac_address);
                }
                // 转换对象
                AdapterInfo {
                    name: adapter.friendly_name().to_string(),
                    index,
                    mac_address: mac_address.unwrap_or("N/A".to_string()),
                    ip_address: parse_address_list_to_string(adapter.ip_addresses()),
                    gateway: parse_address_list_to_string(adapter.gateways()),
                    if_type: adapter.if_type(),
                }
            })
            .collect();

        Ok(adapter_info_vec)
    }

    pub fn get_interface_by_index(&self, index: &u32) -> Result<AdapterInfo, NetRouteError> {
        self.get_interfaces()
            .map_err(|e| NetRouteError::new(e.to_string()))
            .and_then(|adapters| {
                adapters
                    .into_iter()
                    .find(|adapter| adapter.index == *index)
                    .ok_or(NetRouteError::new(format!(
                        "Adapter with index {} not found",
                        index
                    )))
            })
    }

    pub fn get_ipv4_gateway(adapter: &AdapterInfo) -> Result<IpAddr, NetRouteError> {
        // 获取网关地址
        let gateway = adapter
            .gateway
            .parse::<IpAddr>()
            .map_err(|e| NetRouteError::new(e.to_string()))?;
        Ok(gateway)
    }
}

fn find_interface_index_by_mac(
    network_interfaces: &Vec<NetworkInterface>,
    mac_address: &Option<String>,
) -> u32 {
    let mac_address = match mac_address {
        Some(mac) => mac,
        None => return 0,
    };
    network_interfaces
        .iter()
        .find(|interface| {
            if let Some(mac) = &interface.mac_addr {
                mac.to_uppercase() == mac_address.to_uppercase()
            } else {
                false
            }
        })
        .map(|interface| interface.index)
        .unwrap_or(0)
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
fn parse_if_type(if_type: IfType) -> String {
    match if_type {
        IfType::Other => String::from("其他"),
        IfType::EthernetCsmacd => String::from("以太网"),
        IfType::Iso88025Tokenring => String::from("令牌环"),
        IfType::Ppp => String::from("点对点协议"),
        IfType::SoftwareLoopback => String::from("软件回环"),
        IfType::Atm => String::from("ATM"),
        IfType::Ieee80211 => String::from("无线局域网"),
        IfType::Tunnel => String::from("隧道"),
        IfType::Ieee1394 => String::from("IEEE 1394"),
        IfType::Unsupported => String::from("不支持"),
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
fn parse_mac_address(mac_address: Option<&[u8]>) -> Option<String> {
    mac_address
        // 将 u8 数组转换为十六进制字符串，中间用冒号分隔
        .map(|mac| {
            mac.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join(":")
        })
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
        "INDEX",
        "网卡名称",
        "IP地址",
        "MAC地址",
        "网关地址"
    ]);
    for adapter in adapters {
        table.add_row(row![
            parse_if_type(adapter.if_type),
            adapter.index,
            adapter.name,
            adapter.ip_address,
            adapter.mac_address,
            adapter.gateway,
        ]);
    }
    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests;
