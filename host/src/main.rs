use methods::{ ERC20_PROOF_ID, NATIVE_PROOF_ID };
use proof_core::eth_utils::{ ContractRequest, NativeRequest };

mod ethereum;
mod file_utils;
mod prover;

use ethereum::rpc::{ get_contract_input, get_native_input };
use file_utils::{ write_json, parse_json_native, parse_json_contract };
use prover::{ check_signature, run_contract_prover, run_native_prover };
use clap::{ Command, Arg };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("prove-assets")
        .version("1.0")
        .author("Nicolas Le Bel")
        .about("CLI tool to process JSON input and execute prove_eth or prove_erc commands")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input JSON file")
                .required(true)
        )
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .value_name("COMMAND")
                .help("Sets the command to execute: prove_eth or prove_erc")
                .required(true)
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap().as_str();
    let command = matches.get_one::<String>("command").unwrap().as_str();

    match command {
        "prove_eth" => {
            let request = parse_json_native(input_file)?;
            prove_native(request);
        }
        "prove_erc" => {
            let request = parse_json_contract(input_file)?;
            prove_contract(request);
        }
        _ => {
            eprintln!("Invalid command. Please use 'prove_eth' or 'prove_erc'.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn prove_native(input: NativeRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(&input.signature, &input.message, &input.user_address)?);
    println!("Signature corresponds to address {}", &input.user_address);

    println!("Requesting latest account proof for {}", input.user_address);

    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_input_body = get_native_input(input)?;
    println!("Response successfully received.");

    println!("Generating STARK proof of assets...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_native_prover(proof_input_body);

    // Verify receipt seal
    receipt.verify(&NATIVE_PROOF_ID).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!(
        "STARK receipt successfully produced and committed to: {:x?}",
        "./target/proofs/receipt.json"
    );

    Ok(())
}

fn prove_contract(input: ContractRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(&input.signature, &input.message, &input.user_address).unwrap());
    println!("Signature corresponds to address {}", &input.user_address);

    println!(
        "Requesting latest balance slot proof for {} on contract {}",
        &input.user_address,
        &input.contract_address
    );
    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_body = get_contract_input(input).unwrap();
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