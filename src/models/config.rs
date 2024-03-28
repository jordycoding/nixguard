use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(flatten)]
    pub configs: HashMap<String, InterfaceConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct InterfaceConfig {
    // ips: Vec<&str>,
    // listenPort: usize,
    // postSetup: &str,
    pub subnet: [u8; 3],
    pub current_ip: u8,
    pub privateKey: String,
}
