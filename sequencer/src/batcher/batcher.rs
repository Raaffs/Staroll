use std::time::Duration;
use tokio::sync::mpsc;
use crate::MerkleTree;
use crate::{RootPayload, batcher::signer::DigitalSignatureService, crypto::merkle};
pub struct BatcherConfig {
    pub max_batch_size: usize,
    pub max_wait_time: Duration,
}


pub struct BatcherEngine {
    config: BatcherConfig,
    receiver: mpsc::Receiver<RootPayload>,
    digital_signer: Box<dyn DigitalSignatureService>
}

impl BatcherEngine {
    pub fn new(config: BatcherConfig, receiver: mpsc::Receiver<RootPayload>,digital_signer: Box<dyn DigitalSignatureService>) -> Self {
        Self { config, receiver, digital_signer }
    }

    pub async fn run(mut self) {
        let mut current_batch = Vec::with_capacity(self.config.max_batch_size);
        
        // Pin the sleep timer to the heap so we can reset it dynamically
        let sleep_timer = tokio::time::sleep(self.config.max_wait_time);
        tokio::pin!(sleep_timer);

        println!("Batcher active. Limit: {}, Timeout: {:?}", self.config.max_batch_size, self.config.max_wait_time);

        loop {
            tokio::select! {
                // Scenario A: New payload arrives via the network RPC
                Some(payload) = self.receiver.recv() => {
                    // If it's the first item in a new batch, start/reset the timeout clock
                    if current_batch.is_empty() {
                        sleep_timer.as_mut().reset(tokio::time::Instant::now() + self.config.max_wait_time);
                    }
                    
                    current_batch.push(payload);

                    // Trigger processing if size limit is reached
                    if current_batch.len() >= self.config.max_batch_size {
                        println!("Batch full (size limit hit). Sending to pipeline.");
                        self.process_pipeline(&current_batch);
                        current_batch.clear();
                    }
                }

                // Scenario B: Time limit expires before the batch fills up
                _ = &mut sleep_timer, if !current_batch.is_empty() => {
                    println!("Batch timeout expired. Processing partial batch of size: {}", current_batch.len());
                    self.process_pipeline(&current_batch);
                    current_batch.clear();
                }
            }
        }
    }

    // Connects the Batcher to the Merkle and Prover micro-steps
    fn process_pipeline(&self, batch: &[RootPayload]) {
        let mut valid_batch: Vec<[u8;32]> = Vec::new();

        for payload in batch{
            let root_hash: &[u8;32]= match payload.certificate_root.as_slice().try_into(){
                Ok(hash) => hash,
                Err(_) => continue, // Invalid size? Skip to the next loop iteration
            };
            
            match self.digital_signer.verify(&payload.public_key, root_hash, &payload.signature){
                Ok(true)=>{
                    valid_batch.push(root_hash.clone());
                }
                Ok(false)=>{
                    eprintln!("Invalid signature detected, dropping payload.");
                }
                Err(e) => {
                    eprintln!("Signer error during verification: {}, dropping payload.", e);
                }
            }

        }
        
        let tree =MerkleTree::from_leaves(valid_batch);
        let p=tree.build_proof_for(&[1usize]);
    }
}