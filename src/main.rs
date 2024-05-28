use ic_ledger_types::{Memo, Tokens};
use hasher::ledger::{Transaction, TimeStamp, AccountIdentifier, LedgerTransaction};


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

    let tx_hash = Transaction::new(
        from_account,
        to_account,
        None,
        Tokens::from_e8s(48980000),
        Tokens::from_e8s(10000),
        Memo(0),
        TimeStamp {
            timestamp_nanos: 1716563439433251852,
        },
    ).hash();

    // Print the hash
    println!("Tx Hash: {}", tx_hash);
}
