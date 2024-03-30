use crate::{
    client_config::generate_client_config,
    keys::generate_keypair,
    models::config::{AppConfig, InterfaceConfig, PeerConfig, ServerConfig},
};
use std::{
    collections::HashMap,
    fs::{self, write},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub fn add_client(name: &str) {
    let path = Path::new("config.toml");
    let mut server_config: Option<ServerConfig> = None;
    let mut server_keypair: Option<(String, String)> = None;

    let (client_priv_key, client_pub_key) = generate_keypair();

    if !path.exists() {
        server_keypair = Some(generate_keypair());
        println!("No configuration found, generating base config");
        server_config = Some(generate_base_config((
            &server_keypair.clone().unwrap().0,
            &server_keypair.clone().unwrap().1,
        )));
    } else {
        server_config = Some(read_config());
    }

    if let Some(ref mut config) = server_config {
        let interface_config = config.configs.get("wg0");
        if let Some(wg_config) = interface_config {
            let priv_key_server = fs::read_to_string(wg_config.private_key_file.to_string())
                .expect("Couldn't read private key");
            server_keypair = Some((priv_key_server, config.app_config.public_key.to_string()));

            let client_config = generate_client_config(
                client_priv_key,
                &server_keypair.clone().unwrap().1,
                "0.0.0.0",
            );
            let peer_config = PeerConfig {
                public_key: "test pubkey".to_string(),
                allowed_IPs: vec!["test".to_string()],
                name: name.to_owned(),
            };
            client_config.write_to_file("client.conf").unwrap();

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
            println!("Generating config for client: {}", name);
            let mut clone = wg_config.clone();
            clone.peers = Some(peers);
            config.configs = HashMap::from([("wg0".to_owned(), clone)]);
        }
    }
    write("config.toml", toml::to_string(&server_config).unwrap());
}

pub fn generate_base_config((private_key, public_key): (&str, &str)) -> ServerConfig {
    let output = Command::new("wg")
        .arg("genkey")
        .output()
        .expect("Error generating wireguard key");

    write("privkey", output.stdout).expect("Error saving private key");

    let interface_config = InterfaceConfig {
        ips: vec!["10.100.0.1/24".to_owned()],
        listen_port: 51820,
        private_key_file: "privkey".to_owned(),
        peers: None,
    };
    let app_config = AppConfig {
        subnet: [10, 100, 0],
        current_ip: 1,
        public_key: public_key.to_owned(),
    };
    let map = HashMap::from([("wg0".to_string(), interface_config)]);
    ServerConfig {
        app_config: app_config,
        configs: map,
    }
}

pub fn read_config() -> ServerConfig {
    let config_string =
        fs::read_to_string("config.toml").expect("Error reading configuration file");
    toml::from_str(&config_string).expect("Something went wrong parsing the configuration file")
}
