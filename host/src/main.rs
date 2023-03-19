use methods::{ MPT_PROOF_ID };
use dotenv;

mod prover;
mod file_utils;
mod ethereum;

use prover::{ run_prover, check_signature };
use file_utils::write_json;
use ethereum::rpc::get_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load variables in .env
    dotenv::dotenv().ok();

    let address = dotenv::var("ADDRESS").unwrap();
    let signature = dotenv::var("SIGNATURE").unwrap();
    let message = dotenv::var("MESSAGE").unwrap();
    let message = format!("{}{}{}", "\x19Ethereum Signed Message:\n", message.len(), message);

    let provider = dotenv::var("ETHEREUM_PROVIDER").unwrap();
    let block_number = dotenv::var("BLOCK_NUMBER").unwrap_or("latest".to_string());

    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(&signature, &message, &address).unwrap());
    println!("Signature corresponds to address {}", &address);

    println!("Requesting latest account proof for {}", address);
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_input(&provider, &address, &block_number, &signature, &message).unwrap();
    println!("Response successfully received.");

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&MPT_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!("STARK receipt successfully and committed to: {:x?}", "./target/proofs/receipt.json");

    Ok(())
}