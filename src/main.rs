mod models;

use models::config::InterfaceConfig;
use std::{collections::HashMap, process::Command};

use clap::{Parser, Subcommand};

use crate::models::config::ServerConfig;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage client configs
    Clients {
        #[arg(short, long)]
        create: bool,
    },
    /// Manage server configs
    Server {
        #[arg(short, long)]
        create: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Server { create }) => {
            if *create {
                println!("Creating server")
            } else {
                println!("Not creating server")
            }
        }
        Some(Commands::Clients { create }) => {
            if *create {
                let output = Command::new("wg")
                    .arg("genkey")
                    .output()
                    .expect("Error generating wireguard key");

                let config = InterfaceConfig {
                    subnet: [10, 100, 0],
                    current_ip: 12,
                    privateKey: String::from_utf8(output.stdout).unwrap(),
                };
                let map = HashMap::from([("wg0".to_owned(), config)]);
                let server_config = ServerConfig { configs: map };
                println!("{}", toml::to_string(&server_config).unwrap());
            } else {
                println!("Not creating server")
            }
        }
        None => {}
    }
}
