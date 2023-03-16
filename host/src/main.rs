#[macro_use]
extern crate dotenv_codegen;
use methods::{ MPT_PROOF_ID };

mod prover;
mod file_utils;
mod ethereum;

use prover::run_prover;
use file_utils::write_json;
use ethereum::rpc::get_input;

fn main() {
    let address = dotenv!("ADDRESS");
    let signature = dotenv!("SIGNATURE");
    let message = dotenv!("MESSAGE");

    println!("Requesting latest account proof for {}", address);
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_input(
        dotenv!("ETHEREUM_PROVIDER"),
        address,
        "latest",
        signature,
        message
    ).unwrap();
    println!("Response: {:x?}", proof_body);

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&MPT_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!("STARK receipt successfully and committed to: {:x?}", "./target/proofs/receipt.json");
}