#![no_main]

use eth_trie::Trie;
use proof_core::{
    eth_utils::{decode_ethereum_rlp, vec_be_bytes_geq},
    proof_io::{NativeProofInput, NativeProofOutput},
    proof_utils::{create_eth_trie, verify_signed_message},
};
use risc0_zkvm::guest::env;
use sha3::{Digest, Keccak256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let NativeProofInput {user_address, root, block_hash, account_proof, expected_balance, signature, message} = env::read();

    // Verify signed message corresponds to provided address
    // NOTE: Naive ECDSA verification is extremely costly, should be replaced by accelerated circuit
    // as soon as those are made available
    verify_signed_message(&signature, &message, &user_address);

    // Verify Merkle-Patricia trie proof (accountProof in eth_getProof)
    let trie = create_eth_trie();
    let key = Keccak256::digest(&user_address).to_vec();
    let result = trie
        .verify_proof((&root).into(), &key, account_proof)
        .unwrap()
        .unwrap();

    let mut result = decode_ethereum_rlp(result.as_slice()).unwrap();

    // balance is second element in the returned array
    let balance = result.swap_remove(1);
    if vec_be_bytes_geq(&balance, &expected_balance.to_be_bytes().to_vec()) {
        panic!("Account balance is smaller than the expected balance.");
    }

    env::commit(
        &(NativeProofOutput {
            root,
            block_hash,
            expected_balance,
            message,
        }),
    );
}
