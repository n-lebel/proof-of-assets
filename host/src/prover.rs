use methods::{ MPT_PROOF_ID, MPT_PROOF_ELF };
use proof_core::{ ProofInput };
use risc0_zkvm::{ Prover, Receipt, serde::to_vec };

use proof_core::{ eth_utils::{ recover_public_key, derive_address } };
use prefix_hex::decode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run_prover(request: ProofInput) -> Receipt {
    let mut prover = Prover::new(MPT_PROOF_ELF, MPT_PROOF_ID).expect(
        "Prover should be constructed from valid method source code and corresponding image ID"
    );

    // Next we send input to the guest
    prover.add_input_u32_slice(to_vec(&request).expect("Input should be serializable").as_slice());

    let receipt = prover
        .run()
        .expect(
            "Code should be provable unless it had an error or overflowed the maximum cycle count"
        );

    receipt
}

pub fn check_signature(sig: &str, msg: &str, addr: &str) -> Result<bool> {
    let pubkey = derive_address(&recover_public_key(decode(sig).unwrap(), msg.into()).unwrap())?;

    Ok(pubkey == decode::<[u8; 20]>(addr).unwrap())
}