use std::net::SocketAddr;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Play a single player game.
    Single,
    /// Play a two player game.
    Multi {
        #[command(subcommand)]
        command: MultiplayerOptions,
    },
}

#[derive(Debug, Subcommand)]
enum MultiplayerOptions {
    /// Start a server to host game.
    Host {
        /// Address to start server.
        #[arg(short, long, required = true)]
        address: Option<SocketAddr>,
    },
    /// Connect to a server to play.
    Connect {
        /// Address to connect client.
        #[arg(short, long, required = true)]
        address: Option<SocketAddr>,
    },
}

fn main() {
    let _args = Args::parse();
}
