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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EthGetProofBody {
    pub address: [u8; 20],
    pub account_proof: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StorageProof {
    pub key: String,
    pub value: String,
    pub proof: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EthGetBlockBody {
    pub number: String,
    pub state_root: [u8; 32],
}