use anyhow::Result;
use std::path::PathBuf;

use super::executor::Executor;
use crate::bittorrent::bencoded::decode_torrent_file::{decode_torrent_file, info_sha1_hash, Keys};

pub struct InfoExecutor<'a> {
    torrent_path: &'a PathBuf,
}

impl<'a> InfoExecutor<'a> {
    pub fn new(torrent_path: &'a PathBuf) -> Self {
        InfoExecutor { torrent_path }
    }
}

impl<'a> Executor for InfoExecutor<'a> {
    #[allow(unused_variables)]
    fn execute(&self) -> Result<()> {
        let torrent = decode_torrent_file(&self.torrent_path)?;
        println!("{}", format!("Tracker URL: {}", &torrent.announce));
        match &torrent.info.keys {
            Keys::SingleFile { length, md5sum } => {
                println!("{}", format!("Lenght: {}", &length));
            }
            Keys::MultiFiles { files } => {
                let mut lenght = 0;
                for file in files {
                    lenght += file.length;
                }
                println!("{}", format!("Lenght: {:?}", lenght));
            }
        }
        println!("Info Hash: {:?}", &info_sha1_hash(&torrent.info)?);
        println!("Piece Length: {:?}", &torrent.info.piece_length);
        println!("Piece Hashes:");
        torrent
            .info
            .pieces
            .0
            .iter()
            .map(|x| hex::encode(x))
            .for_each(|x| println!("{}", x));
        Ok(())
    }
}
