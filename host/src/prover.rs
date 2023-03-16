use methods::{ MPT_PROOF_ID, MPT_PROOF_ELF };
use proof_core::{ ProofInput };
use risc0_zkvm::{ Prover, Receipt, serde::to_vec };

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