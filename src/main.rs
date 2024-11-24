mod bencoded;

use bencoded::{decode::decode_bencoded_value, decode_torrent_file::decode_torrent_file};
use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        let torrent = decode_torrent_file(&args[2]).unwrap();
        println!("{}", format!("Tracker URL: {}", &torrent.announce));
        println!("{}", format!("Length: {}", &torrent.info.length));
    } else {
        println!("unknown command: {}", args[1])
    }
}
