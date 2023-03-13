#[macro_use]
extern crate dotenv_codegen;
use risc0_zkvm::serde::from_slice;
use methods::{ MPT_PROOF_ID };
use proof_core::ProofOutput;

mod utils;
use utils::{ run_prover, get_input, example_input, write_file };

fn main() {
    let address = "0x904a81b8945803bacbb6c75ab4c956b173975954";

    println!("Requesting latest account proof for {}", address);
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_input(dotenv!("ETHEREUM_PROVIDER"), address, "latest").unwrap();
    println!("Response: {:x?}", proof_body);

    println!("Generating STARK verifying Merkle proof...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_prover(proof_body);

    // Verify receipt seal
    receipt.verify(&MPT_PROOF_ID);
    // Extract journal from receipt
    let output: ProofOutput = from_slice(&receipt.journal).unwrap();

    write_file(receipt, "./receipt.json");
    println!("STARK receipt successfully and committed to: {:x?}", "receipt.json");
}