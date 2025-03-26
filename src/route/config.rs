use crate::base::NetRouteError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteConfig {
    pub ifindex: u32,
    pub domains: Vec<String>,
    pub ips: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteConfigData {
    pub routes: Vec<RouteConfig>,
}

pub fn parse_config_file(json_str: &String) -> Result<RouteConfigData, NetRouteError> {
    let config: RouteConfigData = serde_json::from_str(json_str)
        .map_err(|e| NetRouteError::new(format!("配置文件解析失败: {}", e)))?;
    Ok(config)
}
