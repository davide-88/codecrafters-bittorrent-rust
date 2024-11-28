use anyhow::{Ok, Result};

use super::executor::Executor;
use crate::bittorrent::bencoded::decode::decode_bencoded_value;

pub struct DecodeExecutor<'a> {
    value: &'a String,
}

impl<'a> DecodeExecutor<'a> {
    pub fn new(value: &'a String) -> Self {
        DecodeExecutor { value }
    }
}

impl<'a> Executor for DecodeExecutor<'a> {
    fn execute(&self) -> Result<()> {
        let decoded_value = decode_bencoded_value(&self.value);
        println!("{}", decoded_value.to_string());
        Ok(())
    }
}
