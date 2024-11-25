mod bencoded;
mod executors;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use executors::executor_factory;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Decode {
        // the bencoded value to decode
        value: String,
    },
    /// it decodes a torrent file
    Info {
        /// the path to the torrent file
        torrent_path: PathBuf,
    },
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let cli = Cli::parse();

    let command = &cli.command.expect("Could not find command");
    executor_factory::ExecutorFactory::new()
        .create(command)
        .execute();
}
