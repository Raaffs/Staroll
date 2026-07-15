use serde::Deserialize;
use std::{error::Error, fs, time::Duration};

#[derive(Deserialize)]
pub struct Config {
    pub max_batch_size: usize,
    pub max_wait_ms: u64,
    pub rpc_address: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>>{
        let contents = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn max_wait_duration(&self)->Duration{
        Duration::from_millis(self.max_wait_ms)
    }
}