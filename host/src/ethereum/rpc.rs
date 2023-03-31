use crate::ethereum::requests::{ContractRequest, NativeRequest};
use prefix_hex::{decode, encode, FromHexPrefixed};
use proof_core::{
    eth_utils::{EthGetBlockBody, EthGetProofBody},
    proof_inputs::{ContractProofInput, NativeProofInput},
};

use concat_arrays::concat_arrays;
use serde_json::{Map, Value};
use sha3::{Digest, Keccak256};
use ureq::{agent, Agent};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct EthereumRpcClient {
    client: Agent,
    provider: String,
}

impl EthereumRpcClient {
    pub fn new(provider: &str) -> Self {
        EthereumRpcClient {
            client: agent(),
            provider: provider.to_string(),
        }
    }

    pub fn get_block_by_number(&self, block_number: &str) -> Result<EthGetBlockBody> {
        // eth_getBlockByNumber POST request to the JSON-RPC provider
        let result: Value = self
            .client
            .post(self.provider.as_str())
            .send_json(build_request_payload(
                "eth_getBlockByNumber",
                &[serde_json::json!(block_number), serde_json::json!(false)],
            ))?
            .into_json()?;
        // Parse response as object
        let block_response = result["result"]
            .as_object()
            .expect("eth_getBlockByNumber call failed");

        Ok(parse_block_response(block_response)?)
    }

    pub fn get_proof(
        &self,
        address: &str,
        block_number: &str,
        storage_slot: &str,
    ) -> Result<EthGetProofBody> {
        // eth_getProof POST request to the JSON-RPC provider, with the same block number
        let result: Value = self
            .client
            .post(self.provider.as_str())
            .send_json(build_request_payload(
                "eth_getProof",
                &[
                    serde_json::json!(address),
                    serde_json::json!([storage_slot]),
                    serde_json::json!(block_number),
                ],
            ))?
            .into_json()?;
        // Parse response as object
        let proof_response = result["result"]
            .as_object()
            .expect("eth_getProof call failed");

        Ok(parse_proof_response(proof_response)?)
    }
}

pub fn get_native_input(input: &NativeRequest) -> Result<NativeProofInput> {
    let client = EthereumRpcClient::new(&input.provider);
    let block_response = client.get_block_by_number(&input.block_number)?;
    // for the proof block number, we pass the previous call's response to make sure
    // they are the same (e.g. if "latest" was used there could be a discrepancy)
    let proof_response = client.get_proof(&input.user_address, &block_response.number, "")?;

    let result = NativeProofInput {
        root: block_response.storage_hash,
        block_hash: block_response.block_hash,
        account_proof: proof_response.account_proof,
        user_address: decode_hex_string(&input.user_address),
        expected_balance: input.expected_balance,
        signature: decode_hex_string(&input.signature),
        message: input.message.as_bytes().to_vec(),
    };

    Ok(result)
}

pub fn get_contract_input(input: &ContractRequest) -> Result<ContractProofInput> {
    let client = EthereumRpcClient::new(&input.provider);
    let block_response = client.get_block_by_number(&input.block_number)?;
    // for the proof block number, we pass the previous call's response to make sure
    // they are the same (e.g. if "latest" was used there could be a discrepancy)
    let key_prehash: [u8; 64] = concat_arrays!(
        [0_u8; 12],
        decode_hex_string::<[u8; 20]>(&input.user_address),
        decode_hex_string::<[u8; 32]>(&input.balance_slot)
    );
    let key: String = encode(Keccak256::digest(key_prehash).to_vec());

    let proof_response = client.get_proof(&input.contract_address, &block_response.number, &key)?;

    let result = ContractProofInput {
        storage_hash: proof_response.storage_hash,
        block_hash: block_response.block_hash,
        storage_proof: proof_response.storage_proof,
        user_address: decode_hex_string(&input.user_address),
        contract_address: decode_hex_string(&input.contract_address),
        balance_slot: decode_hex_string(&input.balance_slot),
        expected_balance: input.expected_balance,
        signature: decode_hex_string(&input.signature),
        message: input.message.as_bytes().to_vec(),
    };

    Ok(result)
}

// Helper function for decoding hex strings to Vec<u8>
fn decode_hex_string<T: FromHexPrefixed>(hex: &str) -> T {
    decode(hex).expect("Failed to decode")
}

// Helper function for decoding an array of hex strings to Vec<Vec<u8>>
fn decode_hex_array<T: FromHexPrefixed>(hex_array: &Vec<Value>) -> Vec<T> {
    hex_array
        .iter()
        .map(|hex| decode_hex_string::<T>(hex.as_str().unwrap()))
        .collect::<Vec<_>>()
}

// Function to create the JSON payload for the JSON-RPC request
fn build_request_payload(method: &str, params: &[serde_json::Value]) -> Value {
    ureq::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params,
    })
}

// Function to parse the JSON response of eth_getBlockByNumber into EthGetBlockBody
fn parse_block_response(block_response: &Map<String, Value>) -> Result<EthGetBlockBody> {
    let block_info = EthGetBlockBody {
        number: block_response["number"].as_str().unwrap().to_owned(),
        storage_hash: decode_hex_string(block_response["stateRoot"].as_str().unwrap()),
        block_hash: decode_hex_string(block_response["hash"].as_str().unwrap()),
    };

    Ok(block_info)
}

// Function to parse the JSON response of eth_getProof into EthGetProofBody
fn parse_proof_response(proof_response: &Map<String, Value>) -> Result<EthGetProofBody> {
    // Parse accountProof field to Vec<Vec<u8>>
    let account_proof_json = proof_response["accountProof"].as_array().unwrap();
    let account_proof = decode_hex_array(account_proof_json);

    // Parse storageProof field to Vec<Vec<u8>>: storageProof supports lookup for several keys
    // but we assume there's only one for the time being (hence the [0])
    let storage_proof_json = proof_response["storageProof"][0]["proof"]
        .as_array()
        .unwrap();
    let storage_proof = decode_hex_array(storage_proof_json);

    let storage_hash: [u8; 32] = decode_hex_string(proof_response["storageHash"].as_str().unwrap());
    // Parse address field as [u8; 20]
    let address = decode_hex_string(proof_response["address"].as_str().unwrap());

    let proof_info = EthGetProofBody {
        address,
        account_proof,
        storage_hash,
        storage_proof,
    };

    Ok(proof_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_hex_string() {
        let hex_string = "0x01a3";
        let expected_result: Vec<u8> = vec![1, 163];
        let result = decode_hex_string::<Vec<u8>>(hex_string);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decode_hex_array() {
        let hex_array = vec![serde_json::json!("0x01a3"), serde_json::json!("0x4f52")];
        let expected_result = vec![vec![1, 163], vec![79, 82]];
        let result = decode_hex_array::<Vec<u8>>(&hex_array);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_build_request_payload() {
        let method = "eth_getBlockByNumber";
        let params = &[serde_json::json!("0x1"), serde_json::json!(false)];
        let expected_result = ureq::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["0x1", false],
        });
        let result = build_request_payload(method, params);
        assert_eq!(result, expected_result);
    }
}
