use methods::{ NATIVE_PROOF_ELF, NATIVE_PROOF_ID, ERC20_PROOF_ELF, ERC20_PROOF_ID };
use proof_core::{ NativeProofInput, ContractProofInput };
use risc0_zkvm::{ serde::to_vec, Prover, Receipt };

use prefix_hex::decode;
use proof_core::eth_utils::{ derive_address, recover_public_key };

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run_native_prover(request: NativeProofInput) -> Receipt {
    let mut prover = Prover::new(NATIVE_PROOF_ELF, NATIVE_PROOF_ID).expect(
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

pub fn run_contract_prover(request: ContractProofInput) -> Receipt {
    let mut prover = Prover::new(ERC20_PROOF_ELF, ERC20_PROOF_ID).expect(
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
    let pubkey = derive_address(&recover_public_key(&decode(sig).unwrap(), &msg.into()).unwrap())?;

    Ok(pubkey == decode::<[u8; 20]>(addr).unwrap())
}