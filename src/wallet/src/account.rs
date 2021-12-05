use ed25519_dalek::{Keypair, Signature, PublicKey, Signer};

use crate::TransactionOutput;
use crate::generate_wallet;


// The wallet also acts as an account in oxide
pub struct Wallet {

    // The amount of oxon it holds
    pub balance: u64,

    // The private key from which an account can subtract its public key or
    // it can sign transactions
    pub key_pair: Keypair,

    // The public key of an account
    pub public_key: PublicKey
}

impl Wallet {
    pub fn new() -> Self {
        let key_pair = generate_wallet();
        let public_key = PublicKey::from_bytes(key_pair.public.as_bytes()).unwrap();
        Wallet { balance: 0, key_pair, public_key }
    }

    pub fn sign(&self, transaction: TransactionOutput) -> Signature {
        let transaction_payload = transaction.to_bytes(&self.public_key);
        self.key_pair.sign(transaction_payload.as_slice())
    }
}