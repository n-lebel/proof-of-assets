mod ethereum;
mod file_utils;
mod prover;
mod verifier;

use clap::{Arg, Command};
use file_utils::{parse_json_contract, parse_json_native, parse_json_receipt, write_json};
use methods::{CONTRACT_PROOF_ID, NATIVE_PROOF_ID};
use proof_core::proof_io::{ContractProofOutput, NativeProofOutput};
use prover::prove_assets;
use verifier::verify_receipt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("prove-assets")
        .version("1.0")
        .author("Nicolas Le Bel")
        .about("CLI tool to process JSON input and execute prove_native or prove_contract commands")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input JSON file")
                .required(true),
        )
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .value_name("COMMAND")
                .help("Sets the command to execute: prove_native/prove_contract, or verify_native/verify_contract")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap().as_str();
    let command = matches.get_one::<String>("command").unwrap().as_str();

    match command {
        "prove_native" => {
            let request = parse_json_native(input_file)?;
            let receipt = prove_assets(&request)?;

            write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
            println!(
                "STARK receipt successfully produced and committed to: {:x?}",
                "./target/proofs/receipt.json"
            );
        }
        "prove_contract" => {
            let request = parse_json_contract(input_file)?;
            let receipt = prove_assets(&request)?;

            write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
            println!(
                "STARK receipt successfully produced and committed to: {:x?}",
                "./target/proofs/receipt.json"
            );
        }
        "verify_native" => {
            let receipt = parse_json_receipt(input_file)?;
            let proof_output: NativeProofOutput = verify_receipt(&receipt, &NATIVE_PROOF_ID)?;
            println!("Verified proof successfully!");
            println!("{:x?}", &proof_output);
        }
        "verify_contract" => {
            let receipt = parse_json_receipt(input_file)?;
            let proof_output: ContractProofOutput = verify_receipt(&receipt, &CONTRACT_PROOF_ID)?;
            println!("Verified proof successfully!");
            println!("{:x?}", &proof_output);
        }
        _ => {
            eprintln!("Invalid command. Please use 'prove_eth' or 'prove_erc'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
