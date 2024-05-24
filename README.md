# README

## Transaction Hashing Script

This script is written in Rust and it's used to create a hash of a transaction. The transaction includes details such as the sender, receiver, amount, fee, and timestamp.

### Prerequisites

- Rust programming language installed on your machine. You can download it from [here](https://www.rust-lang.org/tools/install).

### How to Use

1. Clone the repository to your local machine.

2. Open the terminal and navigate to the directory containing the script.

3. Run the script using the command `cargo run`. This will compile and run the script.

The `main` function in the script creates a sample transaction with predefined sender and receiver accounts, and a predefined amount and fee. It then hashes this transaction and prints the hash.

Please note that the sender and receiver accounts are hardcoded in the script as hexadecimal strings. If you want to use different accounts, you need to replace these strings with the hexadecimal representation of your desired accounts.

The script will print an error message and exit if it fails to create the account identifiers from the provided hexadecimal strings.

### Output

The script will print the hash of the transaction to the console. The output will look something like this:

```
Tx Hash: 5c8aea1a5c6b871125c5b876688f2c28483a37314717750f2175156742fd08d8
```

This is the hash of the transaction, which can be used for further processing or verification.