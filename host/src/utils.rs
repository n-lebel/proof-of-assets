use methods::{ MPT_PROOF_ID, MPT_PROOF_ELF };
use proof_core::{ ProofInput, EthGetProofBody, EthGetBlockBody };
use risc0_zkvm::{ Prover, Receipt, serde::to_vec };
use prefix_hex::decode;

use serde_json::Value;
use ureq::agent;
use std::fs::File;
use std::io::prelude::*;

pub fn run_prover(request: ProofInput) -> Receipt {
    let mut prover = Prover::new(MPT_PROOF_ELF, MPT_PROOF_ID).expect(
        "Prover should be constructed from valid method source code and corresponding image ID"
    );

    // Next we send input to the guest
    prover.add_input_u32_slice(to_vec(&request).expect("Input should be serializable").as_slice());

    let receipt = prover
        .run()
        .expect(
            "Code should be provable unless it had an error or overflowed the maximum cycle count"
        );

    receipt
}

pub fn write_file(receipt: Receipt, path: &str) -> std::io::Result<()> {
    let json_str = serde_json::to_string(&receipt)?;

    let mut file = File::create(path)?;
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn get_input(
    provider: &str,
    address: &str,
    block_number: &str,
    signature: &str,
    message: &str
) -> Result<ProofInput, ureq::Error> {
    let block_response = get_block_by_number(provider, block_number).unwrap();
    // for the proof block number, we pass whatever block the previous described to make sure
    // they are the same (e.g. if "latest" was used there could be a discrepancy)
    let proof_response = get_proof(provider, address, &block_response.number).unwrap();

    let result = ProofInput {
        root: block_response.state_root,
        account_proof: proof_response.account_proof,
        account: decode(address).unwrap(),
        expected_balance: 0,
        signature: decode(signature).unwrap(),
        message: decode(message).unwrap(),
    };

    Ok(result)
}

fn get_proof(
    provider: &str,
    address: &str,
    block_number: &str
) -> Result<EthGetProofBody, ureq::Error> {
    // Create an HTTP client
    let agent = agent();

    // eth_getProof POST request to the JSON-RPC provider, with the same block number
    let proof_response: Value = agent
        .post(provider)
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
    let proof_response = proof_response["result"].as_object().unwrap();

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

fn get_block_by_number(provider: &str, block_number: &str) -> Result<EthGetBlockBody, ureq::Error> {
    // Create an HTTP client
    let agent = agent();

    // eth_getBlockByNumber POST request to the JSON-RPC provider
    let block_response: Value = agent
        .post(provider)
        .send_json(
            ureq::json!(    {"jsonrpc": "2.0",
           "id": 1,
           "method": "eth_getBlockByNumber",
           "params": [block_number, false]}
       )
        )?
        .into_json()?;
    // Parse response as object
    let block_response = block_response["result"].as_object().unwrap();
    let block_info = EthGetBlockBody {
        number: block_response["number"].as_str().unwrap().to_owned(),
        state_root: decode(&block_response["stateRoot"].as_str().unwrap().to_owned()).unwrap(),
    };

    Ok(block_info)
}