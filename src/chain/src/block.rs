use std::fmt::{Debug, Formatter, Result};

use wallet::Transaction;

use hex;

use crate::utils::{Hashable, time_now, Hash};
use serde_json;


#[derive(Clone)]
pub struct Block {
    pub number: u32,
    pub timestamp: u128,
    pub last_hash: Hash,
    pub block_hash: Hash,
    pub data: Vec<Transaction>,
    pub nonce: u64
}

impl Block {
    pub fn new(
        timestamp: u128,
        last_hash: Vec<u8>,
        data: Vec<Transaction>,
        number: u32,
    ) -> Self {
        Block {
            number,
            timestamp,
            last_hash,
            block_hash: [0; 16].to_vec(),
            data,
            nonce: 0 as u64
        }
    }

    // Generates the first block in a blockchain
    pub fn genesis() -> Self {
        Block {
            number: 0 as u32,
            timestamp: time_now(),
            last_hash: [0; 16].to_vec(),
            data: Vec::new(),
            block_hash: [0; 16].to_vec(),
            nonce: 0 as u64
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut block_bytes = Vec::new();
        block_bytes.extend(self.last_hash.to_vec());
        block_bytes.extend(&self.timestamp.to_ne_bytes());
        block_bytes.extend(&self.nonce.to_ne_bytes());
        for transaction in self.data.iter() {
            let transaction_bytes = serde_json::to_vec(transaction).unwrap();
            block_bytes.extend(transaction_bytes.as_slice());
        }

        block_bytes

    }

}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "Block #{} time: {}, hash: {}, nonce: {}",
            &self.number,
            &self.timestamp,
            &hex::encode(self.block_hash.as_slice()),
            &self.nonce
        )
    }
}