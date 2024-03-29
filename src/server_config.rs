use crate::models::config::{InterfaceConfig, PeerConfig, ServerConfig};
use std::{collections::HashMap, fs, path::Path, process::Command};

pub fn add_client(name: &str) {
    let path = Path::new("config.toml");
    let mut config: Option<ServerConfig> = None;
    if !path.exists() {
        config = Some(generate_base_config());
    } else {
        let config_string =
            fs::read_to_string("config.toml").expect("Error reading configuration file");
        config = Some(
            toml::from_str(&config_string)
                .expect("Something went wrong parsing the configuration file"),
        );
    }
    let peer_config = PeerConfig {
        public_key: "test pubkey".to_string(),
        allowed_IPs: vec!["test".to_string()],
    };
    match config {
        Some(config) => {
            let interface_config = config.configs.get("wg0");
            let new_config: Option<InterfaceConfig> = match interface_config {
                Some(wg_config) => {
                    let peers = match wg_config.peers.clone() {
                        Some(wg_peers) => {
                            let mut peers_clone = wg_peers.clone();
                            peers_clone.push(peer_config);
                            peers_clone
                        }
                        None => {
                            vec![peer_config]
                        }
                    };
                    let mut clone = wg_config.clone();
                    clone.peers = Some(peers);
                    Some(clone)
                }
                None => None,
            };
            println!("{}", toml::to_string(&new_config.unwrap()).unwrap());
        }
        None => {
            println!("Cant add client")
        }
    }
}

pub fn generate_base_config() -> ServerConfig {
    let output = Command::new("wg")
        .arg("genkey")
        .output()
        .expect("Error generating wireguard key");

    let interface_config = InterfaceConfig {
        ips: vec!["10.100.0.1/24".to_owned()],
        listen_port: 51820,
        subnet: [10, 100, 0],
        current_ip: 1,
        private_key: String::from_utf8(output.stdout)
            .expect("An error occured generating the config"),
        peers: None,
    };
    let map = HashMap::from([("wg0".to_string(), interface_config)]);
    ServerConfig { configs: map }
}
