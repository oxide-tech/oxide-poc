use crate::transaction::Transaction;

pub struct TransactionPool {
    pub transactions: Vec<Transaction>
}
impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction)
    }

    pub fn validate_all(&self) -> bool {
        for t in self.transactions.iter() {
            if !t.verify() {
                return false
            }
        }

        return true
    }
}