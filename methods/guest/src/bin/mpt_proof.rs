#![no_main]

use proof_core::{ ProofInput, ProofOutput };
use risc0_zkvm::guest::env;
use std::sync::Arc;
use eth_trie::{ EthTrie, MemoryDB, Trie };
use rlp::{ DecoderError, Rlp };
use sha3::{ Keccak256, Digest };
use k256::{
    ecdsa::{ recoverable::Signature as r_Signature, VerifyingKey, signature::Signature },
    elliptic_curve::sec1::ToEncodedPoint,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: ProofInput = env::read();

    // Verify signed message corresponds to provided address
    let pubkey = derive_address(&recover_public_key(input.signature, input.message));
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

    // TODO: handle balances larger than u64 (u128 not zkvm-serde-serializable)
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

fn recover_public_key(sig: Vec<u8>, msg: Vec<u8>) -> VerifyingKey {
    let signature: r_Signature = r_Signature::from_bytes(&sig).unwrap();
    signature.recover_verifying_key(&msg).unwrap()
}

fn derive_address(vk: &VerifyingKey) -> [u8; 20] {
    let encoded = vk.to_encoded_point(false);
    let encoded = &encoded.as_bytes()[1..];
    Keccak256::digest(encoded)[12..].try_into().unwrap()
}