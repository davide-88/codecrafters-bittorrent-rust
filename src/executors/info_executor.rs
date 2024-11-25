use anyhow::Result;
use std::path::PathBuf;

use super::executor::Executor;
use crate::bencoded::decode_torrent_file::decode_torrent_file;

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
        Ok(())
    }
}
