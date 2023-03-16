pub mod eth_utils;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProofInput {
    // account address
    pub account: [u8; 20],
    // Ethereum account trie root
    pub root: [u8; 32],
    // Merkle Patricia trie proof for provided account
    pub account_proof: Vec<Vec<u8>>,
    // to prove that the account's balance is larger than some predefined number
    pub expected_balance: u64,
    // used to prove ownership of the account
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProofOutput {
    pub root: [u8; 32],
    pub expected_balance: u64,
}