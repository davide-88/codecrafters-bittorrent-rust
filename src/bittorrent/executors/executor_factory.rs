use crate::Commands;

use super::{decode_executor::DecodeExecutor, executor::Executor, info_executor::InfoExecutor};

pub struct ExecutorFactory {}

impl ExecutorFactory {
    pub fn new() -> Self {
        ExecutorFactory {}
    }

    pub fn create<'a>(&self, command: &'a Commands) -> Box<dyn Executor + 'a> {
        match command {
            Commands::Info { torrent_path } => Box::new(InfoExecutor::new(torrent_path)),
            Commands::Decode { value } => Box::new(DecodeExecutor::new(value)),
        }
    }
}
