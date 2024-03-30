use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub app_config: AppConfig,
    pub netdevs: HashMap<String, NetDev>,
    pub networks: HashMap<String, Network>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDev {
    pub netdev_config: NetDevConfig,
    pub wireguard_config: WireguardConfig,
    pub wireguard_peers: Option<Vec<WireguardPeer>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetDevConfig {
    pub kind: String,
    pub name: String,
    #[serde(rename = "MTUBytes")]
    pub mtu_bytes: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WireguardConfig {
    pub private_key_file: String,
    pub listen_port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WireguardPeer {
    pub wireguard_peer_config: WireguardPeerConfig,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WireguardPeerConfig {
    pub public_key: String,
    #[serde(rename = "AllowedIPs")]
    pub allowed_ips: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub match_config: MatchConfig,
    pub address: Vec<String>,
    pub network_config: NetworkConfig,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchConfig {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    #[serde(rename = "IPMasquerade")]
    pub ip_masquerade: String,
    #[serde(rename = "IPForward")]
    pub ip_forward: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub current_ip: u8,
    pub public_key: String,
}
