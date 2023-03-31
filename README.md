# RISC Zero Ethereum Proof of Assets

`proof-of-assets` allows you to prove ownership of a certain amount of ETH (or any other L1 currency with equivalent account proof structure, e.g. MATIC/AVAX), without divulging owned addresses or exact balances. To achieve this, it first needs a signed message from the claimed address to verify ownership of the private keys, and then verifies a Merkle-Patricia trie proof provided by an RPC endpoint against a public root. It finally verifies that the true balance of the account is equal to or exceeds a claimed balance. If all those conditions are met, it outputs a publicly verifiable RISC Zero STARK.

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

Create a `input.json` file following the `input.example.json` template, and populate the relevant fields. These are as follows:

- `provider`: Ethereum JSON-RPC provider HTTP address
- `user_address`: address of the user whose asset ownership is being proven
- `signature`: an ECDSA secp256k1 signature of the Keccak-hashed eth-formatted message, with v = {00, 01} and not {1b, 1c}
- `message`: a non-formatted string corresponding to the aforementioned signature
- `block_number`: the block number to prove assets against. "latest" will provide the latest block
- `expected_balance`: the claimed owned balance. Needs to be smaller or equal to the actual balance

And for proving values of contract slots, add the following fields:

- `contract_address`: the address of the contract
- `balance_slot`: the slot of the `balances` mapping. The actual slot will be `keccak(abi.encode(address, uint256(balance_slot)))`

Then, simply run the following command to execute the zk-STARK proving algorithm.

```
cargo run --release -- --input <INPUT_FILE (input.json)> --command <prove_eth OR prove_erc>
```

The program outputs a receipt file in `target/proofs/receipt.json`, which contains a seal (the STARK itself), and a journal which is made of the serialized public inputs: the account trie root, the block hash, the claimed balance, the message, and if applicable the contract address and corresponding balance slot.
