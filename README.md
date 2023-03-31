# RISC Zero Ethereum Proof of Assets

`proof-of-assets` allows you to prove ownership of a certain amount of ETH (or any other L1 currency with equivalent account proof structure, e.g. MATIC/AVAX), without divulging owned addresses or exact balances. To achieve this, it first needs a signed message from the claimed address to verify ownership of the private keys, and then verifies a Merkle-Patricia trie proof provided by an RPC endpoint against a public root. It finally verifies that the true balance of the account is equal to or exceeds a claimed balance. If all those conditions are met, it outputs a publicly verifiable RISC Zero STARK.

## Prerequisites

Before using this tool, ensure you have the following software and tools installed:

- _Rust_ (with `rustup`)
- _Nightly Rust_ (automatically installed by cargo using the `rust-toolchain` file)

## Quick start

#### Configuration

Create an input file (e.g. `input.json`) file following the `input.example.json` template, and populate the relevant fields. These are as follows:

- `provider`: Ethereum JSON-RPC provider HTTP address
- `user_address`: address of the user whose asset ownership is being proven
- `signature`: an ECDSA secp256k1 signature of the Keccak-hashed eth-formatted message, with v = {00, 01} and not {1b, 1c}
- `message`: a non-formatted string corresponding to the aforementioned signature
- `block_number`: the block number to prove assets against. "latest" will provide the latest block
- `expected_balance`: the claimed owned balance. Needs to be smaller or equal to the actual balance

And for proving values of contract slots, add the following fields:

- `contract_address`: the address of the contract
- `balance_slot`: the slot of the `balances` mapping. The actual slot will be `keccak(abi.encode(address, uint256(balance_slot)))`

#### Commands

Two proving modes are available, `prove_eth` and `prove_erc`. They are used in the following way:

- `prove_eth` allows you to prove ownership of native assets on Ethereum-equivalent chains. For example, ownership of ETH on Ethereum mainnet
- `prove_contract` allows you to prove ownership of contract-based assets on Ethereum-equivalent chains. For example, for an ERC-20 token, you would want to prove that the balance slot associated to your address holds a given value

#### Proving

To execute the zk-STARK proving algorithm, simply run the following command within the repo:

```
cargo run --release -- --input <INPUT_FILE (input.json)> --command <prove_eth OR prove_erc>
```

The program outputs a receipt file in `target/proofs/receipt.json`, which contains a seal (the STARK itself), and a journal which is made of the serialized public inputs: the account trie root, the block hash, the claimed balance, the message, and if applicable the contract address and corresponding balance slot.
