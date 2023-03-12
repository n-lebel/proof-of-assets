use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProofInput {
    pub root: [u8; 32],
    pub account_proof: Vec<Vec<u8>>,
    pub key: [u8; 32],
    pub expected_balance: u64,
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