use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crypto_hash::{Algorithm, digest};

pub fn time_now() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub(crate) type Hash = Vec<u8>;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        digest(Algorithm::SHA256, &self.bytes())
    }
}