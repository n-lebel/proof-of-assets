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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    #[ignore]
    pub fn test_prove_verify() -> Result<(), Box<dyn std::error::Error>> {
        // get input and prove the assertion
        let input_file = "/Users/nlb/Documents/rust/projects/proof-of-assets/input/input.json";
        let request = parse_json_native(input_file)?;
        let receipt = prove_assets(&request)?;

        // create a temp file, write the receipt and load it back in
        let mut temp_file = NamedTempFile::new().expect("Unable to create temporary file");
        let json = serde_json::to_string(&receipt).unwrap();
        temp_file
            .write_all(json.as_bytes())
            .expect("Unable to write to temporary file");
        let parsed_receipt = parse_json_receipt(temp_file.path().to_str().unwrap())?;
        let _output: NativeProofOutput = verify_receipt(&parsed_receipt, &NATIVE_PROOF_ID)?;

        // check that both the original and receovered receipts match
        assert_eq!(receipt.journal, parsed_receipt.journal);
        assert_eq!(receipt.seal, parsed_receipt.seal);

        Ok(())
    }
}
