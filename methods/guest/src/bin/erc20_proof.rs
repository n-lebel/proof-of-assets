#![no_main]

use concat_arrays::concat_arrays;
use eth_trie::Trie;
use proof_core::{
    eth_utils::be_bytes_geq,
    proof_io::{ContractProofInput, ContractProofOutput},
    proof_utils::{create_eth_trie, verify_signed_message},
};
use risc0_zkvm::guest::env;
use sha3::{Digest, Keccak256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: ContractProofInput = env::read();

    // Verify signed message corresponds to provided address
    // NOTE: Naive ECDSA verification is extremely costly, should be replaced by accelerated circuit
    // as soon as those are made available for Risc0
    verify_signed_message(&input.signature, &input.message, &input.user_address);

    // Compute storage key: for balance mapping, it's Keccak(abi.encode(mapping_key, uint256(mapping_slot)))
    let key_prehash: [u8; 64] = concat_arrays!([0_u8; 12], input.user_address, input.balance_slot);
    let key_prehash = Keccak256::digest(&key_prehash);
    let key = Keccak256::digest(&key_prehash).to_vec();
    // Verify Merkle-Patricia trie proof (accountProof in eth_getProof)
    let trie = create_eth_trie();
    // Slot contents is automatically decoded and should contain balance
    let balance = trie
        .verify_proof((&input.storage_hash).into(), &key, input.storage_proof)
        .unwrap()
        .unwrap();

    let expected_balance = input.expected_balance.to_be_bytes();
    if be_bytes_geq(&balance, &expected_balance) {
        panic!("Account balance is smaller than the expected balance.");
    }

    env::commit(
        &(ContractProofOutput {
            storage_hash: input.storage_hash,
            block_hash: input.block_hash,
            expected_balance: input.expected_balance,
            contract_address: input.contract_address,
            balance_slot: input.balance_slot,
            message: input.message,
        }),
    );
}
