use dotenv;
use methods::{ ERC20_PROOF_ID, NATIVE_PROOF_ID };
use proof_core::eth_utils::format_eth_message;

mod ethereum;
mod file_utils;
mod prover;

use ethereum::rpc::{ get_contract_input, get_native_input };
use file_utils::write_json;
use prover::{ check_signature, run_contract_prover, run_native_prover };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prove_contract()?;
    Ok(())
}

fn prove_native() -> Result<(), Box<dyn std::error::Error>> {
    // Load variables in .env
    dotenv::dotenv().ok();

    let user_address = dotenv::var("USER_ADDRESS").unwrap();
    let signature = dotenv::var("SIGNATURE").unwrap();
    let message = format_eth_message(dotenv::var("MESSAGE").unwrap());

    let contract_address = dotenv::var("CONTRACT_ADDRESS").unwrap();
    let balance_slot = dotenv::var("BALANCE_SLOT").unwrap();

    let provider = dotenv::var("ETHEREUM_PROVIDER").unwrap();
    let block_number = dotenv::var("BLOCK_NUMBER").unwrap_or("latest".to_string());

    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(&signature, &message, &user_address).unwrap());
    println!("Signature corresponds to address {}", &user_address);

    println!(
        "Requesting latest balance slot proof for {} on contract {}",
        user_address,
        contract_address
    );
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_contract_input(
        &provider,
        &user_address,
        &block_number,
        &signature,
        &message,
        &contract_address,
        &balance_slot
    ).unwrap();
    println!("Response successfully received.");

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_contract_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&NATIVE_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!(
        "STARK receipt successfully produced and committed to: {:x?}",
        "./target/proofs/receipt.json"
    );

    Ok(())
}

fn prove_contract() -> Result<(), Box<dyn std::error::Error>> {
    // Load variables in .env
    dotenv::dotenv().ok();

    let user_address = dotenv::var("USER_ADDRESS").unwrap();
    let signature = dotenv::var("SIGNATURE").unwrap();
    let message = format_eth_message(dotenv::var("MESSAGE").unwrap());

    let provider = dotenv::var("ETHEREUM_PROVIDER").unwrap();
    let block_number = dotenv::var("BLOCK_NUMBER").unwrap_or("latest".to_string());

    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(&signature, &message, &user_address).unwrap());
    println!("Signature corresponds to address {}", &user_address);

    println!("Requesting latest account proof for {}", user_address);
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_native_input(
        &provider,
        &user_address,
        &block_number,
        &signature,
        &message
    ).unwrap();
    println!("Response successfully received.");

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_native_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&ERC20_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!(
        "STARK receipt successfully produced and committed to: {:x?}",
        "./target/proofs/receipt.json"
    );

    Ok(())
}