use std::path::PathBuf;

use crate::bencoded::decode_torrent_file::decode_torrent_file;

use super::executor::Executor;

pub struct InfoExecutor<'a> {
    torrent_path: &'a PathBuf,
}

impl<'a> InfoExecutor<'a> {
    pub fn new(torrent_path: &'a PathBuf) -> Self {
        InfoExecutor { torrent_path }
    }
}

impl<'a> Executor for InfoExecutor<'a> {
    fn execute(&self) {
        let result = decode_torrent_file(&self.torrent_path);
        match result {
            Ok(torrent) => {
                println!("{}", format!("Tracker URL: {}", &torrent.announce));
                match &torrent.info.keys {
                    crate::bencoded::decode_torrent_file::Keys::SingleFile { length, md5sum } => {
                        println!("{}", format!("Lenght: {}", &length));
                    }
                    crate::bencoded::decode_torrent_file::Keys::MultiFiles { files } => {
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
            }
            Err(e) => {
                let msg = format!("An error occurred decoding the torrent file: {:?}", e);
                panic!("{}", msg);
            }
        }
    }
}
