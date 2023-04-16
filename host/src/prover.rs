use crate::ethereum::requests::Request;

use proof_core::proof_io::ProofInput;
use risc0_zkvm::{serde::to_vec, Prover, Receipt};

use proof_core::eth_utils::check_signature;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn prove_assets<T: Request>(request: &T) -> Result<Receipt> {
    // Check that the provided signature matches the account before running the costly proving algorithm
    assert!(check_signature(
        &request.get_signature(),
        &request.get_message(),
        &request.get_user_address()
    )?);
    println!(
        "Signature corresponds to address {}",
        &request.get_user_address()
    );
    println!(
        "Requesting {} for {}",
        request.get_description(),
        request.get_user_address()
    );

    // get_input queries the ETHEREUM_PROVIDER over HTTP for a state root and account proof for "address"
    let proof_input_body = request.get_proof_input()?;
    println!("Response successfully received.");
    println!("Generating STARK proof of assets...");

    let mut prover = Prover::new(request.get_proof_elf()).expect(
        "Prover should be constructed from valid method source code and corresponding image ID",
    );
    // run_prover runs the verification of the Merkle Patricia proof within the zkVM with the provided prover
    let receipt = run_prover(&proof_input_body, &mut prover)?;

    // Verify receipt seal
    receipt
        .verify(&request.get_proof_id())
        .expect("Unable to verify receipt.");

    Ok(receipt)
}

fn run_prover<T: ProofInput>(input: &T, prover: &mut Prover) -> Result<Receipt> {
    // Next we send input to the guest
    prover.add_input_u32_slice(to_vec(input)?.as_slice());

    let receipt = prover.run()?;

    Ok(receipt)
}
