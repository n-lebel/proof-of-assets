use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProofInput {
    pub root: [u8; 32],
    pub account_proof: Vec<Vec<u8>>,
    pub key: [u8; 32],
    pub expected_balance: u64,
}