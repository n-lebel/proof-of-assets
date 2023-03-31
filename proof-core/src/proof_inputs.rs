use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NativeProofInput {
    // account address
    pub user_address: [u8; 20],
    // Ethereum account trie root
    pub root: [u8; 32],
    // Ethereum block hash
    pub block_hash: [u8; 32],
    // Merkle Patricia trie proof for provided account
    pub account_proof: Vec<Vec<u8>>,
    // to prove that the account's balance is larger than some predefined number
    pub expected_balance: u64,
    // used to prove ownership of the account
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NativeProofOutput {
    pub root: [u8; 32],
    pub expected_balance: u64,
    pub block_hash: [u8; 32],
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContractProofInput {
    // Contract address (not strictly necessary)
    pub contract_address: [u8; 20],
    // Balance mapping slot (padded to bytes32)
    pub balance_slot: [u8; 32],
    // Account address
    pub user_address: [u8; 20],
    // Ethereum account trie root
    pub storage_hash: [u8; 32],
    // Ethereum block hash
    pub block_hash: [u8; 32],
    // Merkle Patricia trie proof for provided account
    pub storage_proof: Vec<Vec<u8>>,
    // To prove that the account's balance is larger than some predefined number
    pub expected_balance: u64,
    // Used to prove ownership of the account
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContractProofOutput {
    pub contract_address: [u8; 20],
    pub storage_hash: [u8; 32],
    pub expected_balance: u64,
    pub block_hash: [u8; 32],
    pub balance_slot: [u8; 32],
    pub message: Vec<u8>,
}

pub trait ProofInput: Serialize {}

impl ProofInput for ContractProofInput {}
impl ProofInput for NativeProofInput {}
