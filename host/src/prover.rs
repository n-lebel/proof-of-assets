use crate::ethereum::requests::Request;
use crate::write_json;

use proof_core::{ ProofInput };
use risc0_zkvm::{ serde::to_vec, Prover, Receipt };

use prefix_hex::decode;
use proof_core::eth_utils::{ derive_address, recover_public_key };

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check_signature(sig: &str, msg: &str, addr: &str) -> Result<bool> {
    let pubkey = derive_address(&recover_public_key(&decode(sig).unwrap(), &msg.into()).unwrap())?;

    Ok(pubkey == decode::<[u8; 20]>(addr).unwrap())
}

pub fn prove_assets<T>(input: T) -> Result<()> where T: Request {
    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(
        check_signature(&input.get_signature(), &input.get_message(), &input.get_user_address())?
    );
    println!("Signature corresponds to address {}", &input.get_user_address());

    println!("Requesting {} for {}", input.get_description(), input.get_user_address());

    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_input_body = input.get_proof_input()?;
    println!("Response successfully received.");

    println!("Generating STARK proof of assets...");
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM
    let receipt = run_prover(&input, &proof_input_body);

    // Verify receipt seal
    receipt.verify(&input.get_proof_id()).expect("Unable to verify receipt.");

    write_json(&receipt, "./target/proofs").expect("Failed to write to file.");
    println!(
        "STARK receipt successfully produced and committed to: {:x?}",
        "./target/proofs/receipt.json"
    );

    Ok(())
}

fn run_prover<T, S>(request: &T, input: &S) -> Receipt where T: Request, S: ProofInput {
    let mut prover = Prover::new(request.get_proof_elf(), request.get_proof_id()).expect(
        "Prover should be constructed from valid method source code and corresponding image ID"
    );

    // Next we send input to the guest
    prover.add_input_u32_slice(to_vec(input).expect("Input should be serializable").as_slice());

    let receipt = prover
        .run()
        .expect(
            "Code should be provable unless it had an error or overflowed the maximum cycle count"
        );

    receipt
}