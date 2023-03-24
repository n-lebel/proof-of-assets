use dotenv;
use methods::{ NATIVE_PROOF_ID, ERC20_PROOF_ID };

mod ethereum;
mod file_utils;
mod prover;

use ethereum::rpc::{ get_native_input, get_contract_input };
use file_utils::write_json;
use prover::{ check_signature, run_native_prover, run_contract_prover };

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
    // assert!(check_signature(&signature, &message, &address).unwrap());
    // println!("Signature corresponds to address {}", &address);

    println!("Requesting latest account proof for {}", address);
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_contract_input(
        &provider,
        &address,
        &block_number,
        &signature,
        &message,
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "0x0000000000000000000000000000000000000000000000000000000000000009"
    ).unwrap();
    println!("Response successfully received.");

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_contract_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&ERC20_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!(
        "STARK receipt successfully produced and committed to: {:x?}",
        "./target/proofs/receipt.json"
    );

    Ok(())
}