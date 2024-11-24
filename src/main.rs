mod bencoded;

use bencoded::{decode::decode_bencoded_value, decode_torrent_file::decode_torrent_file};
use std::{env, fmt::format};

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
        match &torrent.info.keys {
            bencoded::decode_torrent_file::Keys::SingleFile { length, md5sum } => {
                println!("{}", format!("Lenght: {}", &length));
            }
            bencoded::decode_torrent_file::Keys::MultiFiles { files } => {
                let mut lenght = 0;
                for file in files {
                    lenght += file.length;
                }
                println!("{}", format!("Lenght: {:?}", lenght));
            }
        }
        /*
        println!("{}", format!("Encoding: {:?}", &torrent.encoding));
        println!("{}", format!("keys: {:?}", &torrent.info.keys));
        println!("{}", format!("Name: {}", &torrent.info.name));
        println!("{}", format!("Pieces: {:?}", &torrent.info.pieces));
        */
    } else {
        println!("unknown command: {}", args[1])
    }
}
