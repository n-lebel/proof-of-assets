#![no_main]

use proof_core::{ ProofInput, ProofOutput };
use risc0_zkvm::guest::env;
use std::sync::Arc;
use eth_trie::{ EthTrie, MemoryDB, Trie };
use rlp::{ DecoderError, Rlp };

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: ProofInput = env::read();

    let memdb = Arc::new(MemoryDB::new(true));
    let trie = EthTrie::new(memdb);
    let result = trie
        .verify_proof((&input.root).into(), &input.key, input.account_proof)
        .unwrap()
        .unwrap();

    let mut result = decode_ethereum_rlp(result.as_slice()).unwrap();

    // TODO: handle balances larger than u64 (u128 not serde-serializable)
    let balance = u64::from_be_bytes(result.remove(1).try_into().unwrap());
    if balance < input.expected_balance {
        panic!("Account balance is smaller than the expected balance.");
    }

    env::commit(&(ProofOutput { root: input.root, expected_balance: input.expected_balance }));
}

fn decode_ethereum_rlp(encoded: &[u8]) -> Result<Vec<Vec<u8>>, DecoderError> {
    let rlp = Rlp::new(encoded);
    let decoded: Vec<Vec<u8>> = rlp.as_list()?;
    Ok(decoded)
}