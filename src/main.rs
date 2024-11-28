mod bencoded;
mod executors;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

/**
Piece hases

In a torrent, a file is split into equally-sized parts called pieces.
A piece is usially 256KB or 1MB in size.

Each piece is assigned a SHA-1 has value.
On public networks, there may be malicious peers that send fake data.
These hash values allow us to verify the integrity of each piece that we'll download.

Piece length and piece hashes are specified in the info dictoroary of the torrent file
under the following keys:
 - piece length: the number of bytes in each piece, an integer
 - pieces: cocatenated SHA-1 hashs of each piece (20 bytes each), a string

The BitTorrent protocol specification has more information about these keys.

In this stage, the tester will expect you program to print piece length and a
list of piece hashes in hexadecimal format.

Here's how the tester will execute your program:
 - ./your_bittorrent.sh info sample.torrent

and here's the putput it expects:

Tracker URL: http://bittorrent-test-tracker.codecrafters.io/announce
Lenght: 92063
Info Hash: d69f91e6b2ae4c542468d1073a71d4ea13879a7f
Piece Length:
Piece Hashes:
*/

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> Result<()> {
    let cli = Cli::parse();

    let command = cli.command.context("No command provided")?;
    executor_factory::ExecutorFactory::new()
        .create(&command)
        .execute()?;
    Ok(())
}
