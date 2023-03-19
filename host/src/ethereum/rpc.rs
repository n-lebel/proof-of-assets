use proof_core::{ ProofInput, eth_utils::{ EthGetProofBody, EthGetBlockBody } };
use prefix_hex::decode;

use serde_json::Value;
use ureq::{ agent, Agent };

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct EthereumRpcClient {
    client: Agent,
    provider: String,
}

impl EthereumRpcClient {
    pub fn new(provider: &str) -> Self {
        EthereumRpcClient { client: agent(), provider: provider.to_string() }
    }

    pub fn get_block_by_number(&self, block_number: &str) -> Result<EthGetBlockBody> {
        // eth_getBlockByNumber POST request to the JSON-RPC provider
        let result: Value = self.client
            .post(self.provider.as_str())
            .send_json(
                ureq::json!(    {"jsonrpc": "2.0",
       "id": 1,
       "method": "eth_getBlockByNumber",
       "params": [block_number, false]}
   )
            )?
            .into_json()?;
        // Parse response as object
        let block_response = result["result"]
            .as_object()
            .expect("eth_getBlockByNumber call failed");
        let block_info = EthGetBlockBody {
            number: block_response["number"].as_str().unwrap().to_owned(),
            state_root: decode(&block_response["stateRoot"].as_str().unwrap().to_owned()).unwrap(),
            block_hash: decode(&block_response["hash"].as_str().unwrap().to_owned()).unwrap(),
        };

        Ok(block_info)
    }

    pub fn get_proof(&self, address: &str, block_number: &str) -> Result<EthGetProofBody> {
        // eth_getProof POST request to the JSON-RPC provider, with the same block number
        let result: Value = self.client
            .post(self.provider.as_str())
            .send_json(
                ureq::json!(
                    ureq::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getProof",
                "params": [address,
                    [],
                    block_number],
            })
                )
            )?
            .into_json()?;
        // Parse response as object
        let proof_response = result["result"].as_object().expect("eth_getProof call failed");

        // Parse accountProof field to Vec<Vec<u8>>
        let account_proof_json = proof_response["accountProof"].as_array().unwrap();
        let account_proof = account_proof_json
            .iter()
            .map(|hex_string| decode(hex_string.as_str().unwrap()).unwrap())
            .collect::<Vec<Vec<u8>>>();

        // Parse address field as [u8; 20]
        let address = decode(&proof_response["address"].as_str().unwrap().to_owned()).unwrap();

        let proof_info = EthGetProofBody {
            address,
            account_proof,
        };

        Ok(proof_info)
    }
}

pub fn get_input(
    provider: &str,
    address: &str,
    block_number: &str,
    signature: &str,
    message: &str
) -> Result<ProofInput> {
    let client = EthereumRpcClient::new(provider);
    let block_response = client.get_block_by_number(block_number)?;
    // for the proof block number, we pass the previous call's response to make sure
    // they are the same (e.g. if "latest" was used there could be a discrepancy)
    let proof_response = client.get_proof(address, &block_response.number)?;

    let result = ProofInput {
        root: block_response.state_root,
        block_hash: block_response.block_hash,
        account_proof: proof_response.account_proof,
        account: decode(address).unwrap(),
        expected_balance: 0,
        signature: decode(signature).unwrap(),
        message: message.as_bytes().to_vec(),
    };

    Ok(result)
}