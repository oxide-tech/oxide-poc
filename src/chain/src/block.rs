use std::fmt::{Debug, Formatter, Result};

use wallet::Transaction;

use crypto_hash::{digest, Algorithm};
use hex;

use crate::utils::time_now;
use serde_json;



pub struct Block {
    pub number: i32,
    pub timestamp: u64,
    pub last_hash: i128,
    pub hash: i128,
    pub data: Vec<Transaction>,
    pub validator: i128
}

impl Block {
    pub fn new(
        timestamp: u64,
        last_hash: i128,
        data: Vec<Transaction>,
        number: i32,
        validator: i128
    ) -> Self{
        Block {
            number,
            timestamp,
            last_hash,
            hash: 0 as i128,
            data,
            validator
        }
    }

    // Generates the first block in a blockchain
    pub fn genesis() -> Self {
        Block {
            number: 0 as i32,
            timestamp: time_now(),
            last_hash: 0 as i128,
            data: Vec::new(),
            hash: 0 as i128,
            validator: 0 as i128
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut block_info = Vec::new();

        block_info.extend(self.last_hash.to_ne_bytes());
        block_info.extend(&self.timestamp.to_ne_bytes());
        for transaction in self.data.iter() {
            let transaction_bytes = serde_json::to_vec(transaction).unwrap();
            block_info.extend(transaction_bytes.as_slice());
        }

        digest(Algorithm::SHA256, &block_info)
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "Block #{} time: {}, prev_hash: {}",
            &self.number,
            &self.timestamp,
            &hex::encode(&self.last_hash.to_ne_bytes()),
        )
    }
}