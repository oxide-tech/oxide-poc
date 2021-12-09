mod pow;
mod utils;

pub use crate::pow::Miner;


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::pow::Miner;
    use chain::Blockchain;


    #[test]
    fn test_miner_creation(){
        let test_blockchain = Blockchain::new();
        let last_block = test_blockchain.last_block();
        let wrapped_chain = Arc::new(Mutex::new(test_blockchain.clone()));
        let miner = Miner::new(&wrapped_chain, 0 as u128);

        assert_eq!(miner.get_last_block().timestamp, last_block.timestamp);
    }
}
