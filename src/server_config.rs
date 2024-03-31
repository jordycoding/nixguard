use colored::Colorize;
use inquire::Text;

use crate::{
    client_config::generate_client_config,
    keys::generate_keypair,
    models::config::{
        AppConfig, MatchConfig, NetDev, NetDevConfig, Network, NetworkConfig, ServerConfig,
        WireguardConfig, WireguardPeer, WireguardPeerConfig,
    },
};
use std::{
    collections::HashMap,
    env,
    fs::{self, write},
    path::Path,
    process::{Command, Stdio},
};

pub fn add_client() {
    let path = Path::new("config.toml");
    let mut server_config: Option<ServerConfig> = None;
    let mut server_keypair: Option<(String, String)> = None;

    let (client_priv_key, client_pub_key) = generate_keypair();

    if !path.exists() {
        server_keypair = Some(generate_keypair());
        println!("No configuration found, generating base config");
        let priv_path = Text::new("What path will the server secret key be stored at?").prompt();
        server_config = Some(generate_base_config(
            &server_keypair.clone().unwrap().0,
            &priv_path.expect("Invalid path"),
        ));
        println!("This is the servers private key, please store this in the specified place, it won't be saved anywhere else");
        println!(
            "{}",
            &server_keypair.clone().unwrap().1.on_red().black().bold()
        );
    } else {
        server_config = Some(read_config());
    }

    let name = Text::new("What is the clients name?").prompt();
    let server_ip = Text::new("What is the servers external ip?").prompt();
    let default_path = match env::var("HOME") {
        Ok(val) => val,
        Err(e) => ".".to_string(),
    };
    let config_path = Text::new("Where should the client configuration be saved")
        .with_default(&default_path)
        .prompt();
    if let Some(ref mut config) = server_config {
        let netdev = config.netdevs.get("50-wg0");
        if let Ok(valid_name) = name {
            if let Some(netdev_config) = netdev {
                let ip = format!("10.100.0.{}", config.app_config.current_ip + 1);
                let client_config = generate_client_config(
                    client_priv_key,
                    &config.app_config.public_key,
                    &ip,
                    &server_ip.expect("Invalid ip"),
                );
                client_config
                    .write_to_file(format!(
                        "{}/client-{}.conf",
                        config_path.unwrap(),
                        valid_name
                    ))
                    .unwrap();

                let wireguard_peer_config = WireguardPeerConfig {
                    public_key: client_pub_key,
                    allowed_ips: vec![ip],
                };
                let wireguard_peer = WireguardPeer {
                    wireguard_peer_config,
                };

                config.app_config.current_ip += 1;

                let peers = match netdev_config.wireguard_peers.clone() {
                    Some(wg_peers) => {
                        let mut peers_clone = wg_peers.clone();
                        peers_clone.push(wireguard_peer);
                        peers_clone
                    }
                    None => {
                        vec![wireguard_peer]
                    }
                };
                println!("Generating config for client: {}", valid_name);
                let mut clone = netdev_config.clone();
                clone.wireguard_peers = Some(peers);
                let new_netdevs = HashMap::from([("50-wg0".to_owned(), clone)]);
                config.netdevs = new_netdevs;
            }
        } else {
            println!("Invalid name");
        }
    }
    write("config.toml", toml::to_string(&server_config).unwrap());
}

pub fn generate_base_config(public_key: &str, priv_key_file: &str) -> ServerConfig {
    let netdev_config = NetDevConfig {
        kind: "wireguard".to_owned(),
        name: "wg0".to_owned(),
        mtu_bytes: "1300".to_owned(),
    };
    let wireguard_config = WireguardConfig {
        private_key_file: priv_key_file.to_owned(),
        listen_port: 51820,
    };
    let netdev = NetDev {
        netdev_config,
        wireguard_config,
        wireguard_peers: None,
    };

    let match_config = MatchConfig {
        name: "wg0".to_owned(),
    };
    let network_config = NetworkConfig {
        ip_masquerade: "ipv4".to_owned(),
        ip_forward: true,
    };
    let network = Network {
        match_config,
        address: vec!["10.100.0.1/24".to_owned()],
        network_config,
    };

    let app_config = AppConfig {
        current_ip: 1,
        public_key: public_key.to_owned(),
    };

    let netdevs = HashMap::from([("50-wg0".to_string(), netdev)]);
    let networks = HashMap::from([("wg0".to_string(), network)]);
    ServerConfig {
        app_config,
        netdevs,
        networks,
    }
}

pub fn read_config() -> ServerConfig {
    let config_string =
        fs::read_to_string("config.toml").expect("Error reading configuration file");
    toml::from_str(&config_string).expect("Something went wrong parsing the configuration file")
}
