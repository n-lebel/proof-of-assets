use crate::ethereum::rpc::{get_contract_input, get_native_input};
use methods::{CONTRACT_PROOF_ELF, CONTRACT_PROOF_ID, NATIVE_PROOF_ELF, NATIVE_PROOF_ID};
use proof_core::proof_io::{ContractProofInput, NativeProofInput, ProofInput};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Request {
    type Input: ProofInput;
    fn get_user_address(&self) -> &str;
    fn get_signature(&self) -> &str;
    fn get_message(&self) -> &str;
    fn get_expected_balance(&self) -> &u64;
    fn get_proof_id(&self) -> [u32; 8];
    fn get_proof_elf(&self) -> &[u8];
    fn get_proof_input(&self) -> Result<Self::Input>;
    fn get_description(&self) -> String;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NativeRequest {
    pub provider: String,
    pub user_address: String,
    pub block_number: String,
    pub signature: String,
    pub message: String,
    pub expected_balance: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContractRequest {
    pub provider: String,
    pub user_address: String,
    pub block_number: String,
    pub signature: String,
    pub message: String,
    pub expected_balance: u64,
    pub contract_address: String,
    pub balance_slot: String,
}

impl Request for NativeRequest {
    type Input = NativeProofInput;

    fn get_user_address(&self) -> &str {
        &self.user_address
    }

    fn get_signature(&self) -> &str {
        &self.signature
    }

    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_expected_balance(&self) -> &u64 {
        &self.expected_balance
    }

    fn get_proof_id(&self) -> [u32; 8] {
        NATIVE_PROOF_ID
    }

    fn get_proof_elf(&self) -> &[u8] {
        &NATIVE_PROOF_ELF
    }

    fn get_proof_input(&self) -> Result<Self::Input> {
        get_native_input(self)
    }

    fn get_description(&self) -> String {
        String::from("latest account proof")
    }
}

impl Request for ContractRequest {
    type Input = ContractProofInput;

    fn get_user_address(&self) -> &str {
        &self.user_address
    }

    fn get_signature(&self) -> &str {
        &self.signature
    }

    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_expected_balance(&self) -> &u64 {
        &self.expected_balance
    }

    fn get_proof_id(&self) -> [u32; 8] {
        CONTRACT_PROOF_ID
    }

    fn get_proof_elf(&self) -> &[u8] {
        &CONTRACT_PROOF_ELF
    }

    fn get_proof_input(&self) -> Result<Self::Input> {
        get_contract_input(self)
    }

    fn get_description(&self) -> String {
        format!(
            "latest balance slot proof for contract {}",
            &self.contract_address
        )
    }
}
