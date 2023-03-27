#![no_main]

use proof_core::{
    NativeProofInput,
    NativeProofOutput,
    eth_utils::{ decode_ethereum_rlp, recover_public_key, derive_address, vec_be_bytes_geq },
};
use risc0_zkvm::guest::env;
use std::sync::Arc;
use eth_trie::{ EthTrie, MemoryDB, Trie };
use sha3::{ Keccak256, Digest };

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: NativeProofInput = env::read();

    // Verify signed message corresponds to provided address
    // NOTE: Naive ECDSA verification is extremely costly, should be replaced by accelerated circuit
    // as soon as those are made available for Risc0
    let pubkey = derive_address(
        &recover_public_key(&input.signature, &input.message).unwrap()
    ).unwrap();
    if pubkey != input.account.to_owned() {
        panic!("Signature does not match provided address.");
    }

    // Verify Merkle-Patricia trie proof (accountProof in eth_getProof)
    let memdb = Arc::new(MemoryDB::new(true));
    let trie = EthTrie::new(memdb);
    let key = Keccak256::digest(&input.account).to_vec();
    let result = trie
        .verify_proof((&input.root).into(), &key, input.account_proof)
        .unwrap()
        .unwrap();

    let mut result = decode_ethereum_rlp(result.as_slice()).unwrap();

    // balance is second element in the returned array
    let balance = result.swap_remove(1);
    let expected_balance = input.expected_balance.to_be_bytes().to_vec();
    if vec_be_bytes_geq(&balance, &expected_balance) {
        panic!("Account balance is smaller than the expected balance.");
    }

    env::commit(
        &(NativeProofOutput {
            root: input.root,
            block_hash: input.block_hash,
            expected_balance: input.expected_balance,
            message: input.message,
        })
    );
}