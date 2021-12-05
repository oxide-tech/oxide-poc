use ed25519_dalek::{PublicKey, Signature, Verifier};
use uuid::Uuid;
use bincode;

use crate::time_now;
use crate::account::Wallet;
use serde;
use serde::ser::{Serialize, Serializer, SerializeStruct};

const TRANSACTION_FEE: u64 = 1 as u64;

#[derive(serde::Serialize, Debug)]
pub enum TransactionType {
    Stake,
    Validator,
    Transaction,
    Unsigned
}

#[derive(Debug)]
pub struct TransactionInput {
    pub timestamp: u64,
    pub sender: PublicKey,
    pub signature: Signature
}

#[derive(Clone, Copy, Debug)]
pub struct TransactionOutput {
    pub receiver: PublicKey,
    pub amount: u64,
    pub fee: u64
}

#[derive(serde::Serialize, Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub input: TransactionInput,
    pub output: TransactionOutput
}

impl TransactionInput {
    pub fn new(sender: PublicKey, signature: Signature) -> Self{
        let transaction_timestamp = time_now();
        TransactionInput { timestamp: transaction_timestamp, sender, signature }
    }
}

impl Serialize for TransactionInput{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("TransactionInput", 3).unwrap();
        state.serialize_field("timestamp", &self.timestamp)?;

        let sender_ser = bincode::serialize(&self.sender).unwrap();
        state.serialize_field("sender", &sender_ser)?;

        let signature_ser = bincode::serialize(&self.signature).unwrap();
        state.serialize_field("signature", &signature_ser)?;

        state.end()
    }
}

impl TransactionOutput {
    pub fn new(receiver: PublicKey, amount: u64) -> Self {
        TransactionOutput {receiver, amount, fee: TRANSACTION_FEE}
    }

    pub fn to_bytes(&self, sender: &PublicKey) -> Vec<u8> {
        let mut bytes_buffer = Vec::new();

        bytes_buffer.extend(&self.receiver.to_bytes());
        bytes_buffer.extend(&self.amount.to_ne_bytes());
        bytes_buffer.extend(&self.fee.to_ne_bytes());
        bytes_buffer.extend(sender.to_bytes());

        bytes_buffer
    }
}

impl Serialize for TransactionOutput{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("TransactionOutput", 3).unwrap();
        state.serialize_field("receiver", &bincode::serialize(&self.receiver).unwrap())?;
        state.serialize_field("amount", &self.amount)?;
        state.serialize_field("fee", &self.fee)?;
        state.end()
    }
}

impl Transaction {
    pub fn new(sender_wallet: Wallet, receiver: PublicKey, amount: u64, transaction_type: TransactionType) -> Result<Self, &'static str> {
        if sender_wallet.balance < amount {
           return Err("Insufficient balance in senders wallet")
        }

        let transaction_id = Uuid::new_v4();
        let transaction_output = TransactionOutput::new(receiver, amount);
        let signature = sender_wallet.sign(transaction_output);
        let transaction_input = TransactionInput::new(sender_wallet.public_key, signature);

        Ok(Transaction {
            id: transaction_id,
            transaction_type,
            input: transaction_input,
            output: transaction_output
        })
    }

    pub fn verify(&self) -> bool {
        let sender_key = self.input.sender;
        let payload = self.output.to_bytes(&sender_key);

        sender_key.verify(payload.as_slice(), &self.input.signature).is_ok()
    }
}