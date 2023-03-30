#![no_main]

use proof_core::{
    proof_inputs::{ NativeProofInput, NativeProofOutput },
    eth_utils::{ decode_ethereum_rlp, vec_be_bytes_geq },
    proof_utils::{ verify_signed_message, create_eth_trie },
};
use risc0_zkvm::guest::env;
use eth_trie::{ Trie };
use sha3::{ Keccak256, Digest };

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: NativeProofInput = env::read();

    // Verify signed message corresponds to provided address
    // NOTE: Naive ECDSA verification is extremely costly, should be replaced by accelerated circuit
    // as soon as those are made available for Risc0
    verify_signed_message(&input.signature, &input.message, &input.user_address);

    // Verify Merkle-Patricia trie proof (accountProof in eth_getProof)
    let trie = create_eth_trie();
    let key = Keccak256::digest(&input.user_address).to_vec();
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