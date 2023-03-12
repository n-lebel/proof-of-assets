#[macro_use]
extern crate dotenv_codegen;
use risc0_zkvm::serde::from_slice;
use proof_core::ProofOutput;

mod utils;
use utils::{ run_prover, get_input, example_input };

fn main() {
    let address = "0x904a81b8945803bacbb6c75ab4c956b173975954";

    println!("Requesting latest account proof for {}", address);
    let proof_body = get_input(dotenv!("ETHEREUM_PROVIDER"), address, "latest").unwrap();
    println!("Response: {:x?}", proof_body);

    println!("Generating STARK verifying Merkle proof...");
    let receipt = run_prover(proof_body);

    // Extract journal of receipt
    let output: ProofOutput = from_slice(&receipt.journal).unwrap();

    println!("STARK receipt successfully verified with output: {:x?}", output);
}