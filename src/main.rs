use clap::{Parser, Subcommand};

#[derive(Parser)]
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
                println!("Creating client")
            } else {
                println!("Not creating client")
            }
        }
        Some(Commands::Clients { create }) => {
            if *create {
                println!("Creating server")
            } else {
                println!("Not creating server")
            }
        }
        None => {}
    }
}
