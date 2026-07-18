mod config;
mod crypto; 
use config::Config;
mod internal;
mod batcher;
use internal::models::models::RootPayload;
use crypto::merkle::MerkleTree;

use crate::crypto::merkle;

fn main() {
    let config = Config::load().expect("Failed to load config");
    println!("max batch size {}", config.max_batch_size);
    println!("max wait ms {}", config.max_wait_ms);
    println!("rpc address {}", config.rpc_address);
    let leaves= Vec::<[u8;32]>::new();
    let tree= MerkleTree::FromLeaves(leaves);
}