use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub app_config: AppConfig,
    pub configs: HashMap<String, InterfaceConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceConfig {
    pub ips: Vec<String>,
    pub listen_port: usize,
    // pub postSetup: String,
    pub private_key_file: String,
    pub peers: Option<Vec<PeerConfig>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeerConfig {
    pub name: String,
    pub public_key: String,
    pub allowed_IPs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub subnet: [u8; 3],
    pub current_ip: u8,
    pub public_key: String,
}
