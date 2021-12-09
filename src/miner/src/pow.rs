use std::sync::{Arc, Mutex};
use std::u128;
use chain::{Block, Blockchain, time_now, Hashable};
use tokio;
use tokio::task::JoinHandle;

use crate::utils::{gen_nonce, hash_as_u128};

pub struct Miner<'a> {
    pub(crate) blockchain: &'a Arc<Mutex<Blockchain>>,
    pub difficulty: u128
}

impl<'a> Miner<'a> {
    pub fn new(blockchain: &'a Arc<Mutex<Blockchain>>, difficulty: u128) -> Self {
        Miner {
            blockchain,
            difficulty
        }
    }

    // Wrapper over `Blockchain::last_block` as it locks the blockchain for read
    // write
    pub fn get_last_block(&self) -> Block {
        let chain_lock = self.blockchain.lock().unwrap();
        chain_lock.last_block().clone()
    }

    pub async fn mine(&self) -> JoinHandle<()> {
        let chain_data = Arc::clone(self.blockchain);
        let difficulty = self.difficulty.clone();

        tokio::spawn(async move {
            loop {
                let last_block = {
                    let chain_lock = chain_data.lock().unwrap();
                    chain_lock.last_block().clone()
                };
                let mut new_block = Block::new(
                    time_now(),
                    last_block.block_hash.clone(),
                    Vec::new(),
                    last_block.number + 1
                );
                loop {
                    new_block.nonce = gen_nonce();
                    let hash = new_block.hash();
                    if difficulty > hash_as_u128(&hash){
                        let mut chain_lock = chain_data.lock().unwrap();
                        new_block.block_hash = hash.clone();
                        println!("INFO > NEW: {:?}", new_block);
                        chain_lock.add_new_block(new_block.clone());
                        break;
                        }
                    }
            }
        })
    }

}