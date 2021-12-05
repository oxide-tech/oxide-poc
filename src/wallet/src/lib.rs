mod transaction;
mod transaction_pool;
mod account;

pub use transaction::{Transaction, TransactionType, TransactionInput, TransactionOutput};
use std::time::{SystemTime, UNIX_EPOCH};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

// Generates the key-pair for a wallet
fn generate_wallet() -> Keypair {
    let mut os_random = OsRng{};
    Keypair::generate(&mut os_random)
}

pub fn time_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}


#[cfg(test)]
mod tests {
    use crate::{transaction, TransactionType};
    use crate::account;
    use crate::generate_wallet;
    use crate::transaction_pool;
    use ed25519_dalek::Verifier;

    #[test]
    fn test_generate_wallet(){
        generate_wallet();
        assert!(true);
    }

    #[test]
    fn create_account(){
        let new_account = account::Wallet::new();
        assert_eq!(new_account.balance, 0);
    }

    #[test]
    fn sign_transaction(){
        let test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();

        let test_transaction_output = transaction::TransactionOutput::new(
            test_account_two.public_key, 12);

        let signature = test_account_one.sign(test_transaction_output);

        let payload = test_transaction_output.to_bytes(&test_account_one.public_key);

        assert!(test_account_one.public_key.verify(payload.as_slice(), &signature).is_ok())
    }

    #[test]
    fn generate_valid_transaction() {
        let mut test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();

        test_account_one.balance = test_account_one.balance + 20;

        let new_transaction = transaction::Transaction::new(
            test_account_one,
            test_account_two.public_key,
            10,
            TransactionType::Transaction
        );

        match new_transaction {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn generate_invalid_transaction(){
        let test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();

        let new_transaction = transaction::Transaction::new(
            test_account_one,
            test_account_two.public_key,
            10,
            TransactionType::Transaction
        );

        match new_transaction {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn validate_transaction(){
        let mut test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();

        test_account_one.balance = test_account_one.balance + 20;

        let new_transaction = transaction::Transaction::new(
            test_account_one,
            test_account_two.public_key,
            10,
            TransactionType::Transaction
        ).unwrap();

        assert_eq!(new_transaction.verify(), true);
    }

    #[test]
    fn validate_transaction_erronated(){
        let mut test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();
        let hacker_account = account::Wallet::new();

        test_account_one.balance = test_account_one.balance + 20;

        let mut new_transaction = transaction::Transaction::new(
            test_account_one,
            test_account_two.public_key,
            10,
            TransactionType::Transaction
        ).unwrap();

        new_transaction.output.receiver = hacker_account.public_key;

        assert_eq!(new_transaction.verify(), false);
    }

    #[test]
    fn validate_transaction_pool(){
        let mut new_pool = transaction_pool::TransactionPool::new();
        let mut test_account_one = account::Wallet::new();
        let test_account_two = account::Wallet::new();

        test_account_one.balance = test_account_one.balance + 20;

        let new_transaction = transaction::Transaction::new(
            test_account_one,
            test_account_two.public_key,
            10,
            TransactionType::Transaction
        ).unwrap();

        new_pool.add_transaction(new_transaction);

        assert_eq!(new_pool.validate_all(), true);
    }
}
