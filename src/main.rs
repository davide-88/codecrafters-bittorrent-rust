mod bittorrent;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use bittorrent::executors::executor_factory;

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
Piece hashes

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

/**
    Discover peers

    Trackers are centra serveres that maintain information abut peers
    participating in the sharing and dowloading of a torrent.

    In this stage, you'll make a GET request to a HTTP tracker to discover
    peers to download the file from.

    Tracker GET request

    You will need t make a request to the tracker URL you extracted in the
    previous stage.

    - info_hash: the info has of the torrent
        * 20 byte long, will need to be URL encoded
        * Note: this is NOT the hexadecimal representation, which is 40 characters long
    - peer_id: a unique identifier for yout client
        * A string of thenght 20 that you get to pick. You can use something like
          "00112233445566778899".
    - port: the port your client is listening on
        * You can set this to 6881 you will not have to support this functionality
          during this challange
    - uploaded: the total amount uploaded so far
        * Since your client hasn't downloaded anything yet you can set this to 0.
    - left: the numebr of butes left to download
        * Since your client hasn't downloaded anything yet, this'll be the total
          length of the file (you have extracted this value from the torrent file
          in the previous stage)
    - compact: whether the peer list shoudl use the compact represetation
        * You can set this to 1
        * the compact representation is more commonly used in the wild, the non-compact
          representation is mostly supported for backward-compatability

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
