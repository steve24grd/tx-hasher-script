use ic_ledger_types::{AccountIdentifier, Memo, Transaction, Timestamp, Operation, Tokens};
use serde_cbor;
use sha2::{Sha256, Digest};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

const HASH_LENGTH: usize = 32;

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashOf<T> {
    hash: [u8; HASH_LENGTH],
    phantom: PhantomData<T>,
}


impl<T> HashOf<T> {
    pub fn new(bs: [u8; HASH_LENGTH]) -> Self {
        HashOf {
            hash: bs,
            phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for HashOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.hash))
    }
}

fn hash_transaction(tx: &Transaction) -> HashOf<Transaction> {
    let serialized = serde_cbor::ser::to_vec_packed(&tx).unwrap();

    let mut state = Sha256::new();
    state.update(&serialized);

    let result = state.finalize();
    let fixed_result: [u8; HASH_LENGTH] = result.into();
    HashOf::new(fixed_result)
}

fn main() {
    // Create sample data
    let from_hex = "4ef6b55f55a4bbeb4646b4e433da030662f4d0c097c5b158bf64ec44dfaeda53";
    let to_hex = "5c8aea1a5c6b871125c5b876688f2c28483a37314717750f2175156742fd08d8";

    let from_account = match AccountIdentifier::from_hex(from_hex) {
        Ok(account) => account,
        Err(e) => {
            eprintln!("Failed to create from account-identifier from hex: {}", e);
            return;
        }
    };

    let to_account = match AccountIdentifier::from_hex(to_hex) {
        Ok(account) => account,
        Err(e) => {
            eprintln!("Failed to create to account-identifier from hex: {}", e);
            return;
        }
    };

    let transaction = Transaction {
        memo: Memo(0),
        operation: Some(Operation::Transfer {
            from: from_account,
            to: to_account,
            amount: Tokens::from_e8s(48980000),
            fee: Tokens::from_e8s(10000),
        }),
        created_at_time: Timestamp {
            timestamp_nanos: 1716563439433251852,
        },
        icrc1_memo: None,
    };

    // Hash the transaction
    let hash = hash_transaction(&transaction);

    // Print the hash
    println!("Tx Hash: {}", hash);
}
