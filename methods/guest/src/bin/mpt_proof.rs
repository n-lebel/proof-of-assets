#![no_main]

use proof_core::{ ProofInput };
use risc0_zkvm::guest::env;
use std::sync::Arc;
use eth_trie::{ EthTrie, MemoryDB, Trie, DB };
use primitive_types::H256;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: ProofInput = env::read();

    let memdb = Arc::new(MemoryDB::new(true));
    let trie = EthTrie::new(memdb);
    let result = trie.verify_proof((&input.root).into(), &input.key, input.account_proof).unwrap();
}