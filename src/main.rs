mod client_config;
mod keys;
mod models;
mod server_config;

use server_config::add_client;
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
#[command(arg_required_else_help = true)]
enum Commands {
    /// Add client
    Add,
    /// List all clients
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add) => {
            // let output = Command::new("wg")
            //     .arg("genkey")
            //     .output()
            //     .expect("Error generating wireguard key");

            add_client();
        }
        Some(Commands::List) => {}
        None => {}
    }
}
