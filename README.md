# RISC Zero Ethereum Proof of Assets

`proof-of-assets` allows you to prove ownership of a certain amount of ETH (or any other L1 currency with equivalent account proof structure, e.g. MATIC/AVAX), without divulging owned addresses or exact balances. To achieve this, it first needs a signed message from the claimed address to verify ownership of the private keys, and then verifies a Merkle-Patricia trie proof provided by an RPC endpoint against a public root. It finally verifies that the true balance of the account is equal to or exceeds a claimed balance. If all those conditions are met, it outputs a publicly verifiable RISC Zero STARK.

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

Create a `.env` file following the `.env.example` template, and populate the relevant fields. Then, simply run the following command to execute the zk-STARK proving algorithm.

```
cargo run --release
```

The program outputs a receipt file in `target/proofs/receipt.json`, which contains a seal (the STARK itself), and a journal which is made of the serialized "public inputs": the account trie root, the block hash, and the claimed balance.
