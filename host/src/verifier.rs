use proof_core::proof_io::ProofOutput;
pub use risc0_zkvm::{serde::from_slice, Receipt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn verify_receipt<T: ProofOutput>(receipt: &Receipt, image_id: &[u32; 8]) -> Result<T> {
    // Verify receipt seal
    receipt.verify(image_id)?;

    // Extract the proof output
    let journal = receipt.get_journal_bytes();

    Ok(from_slice(journal)?)
}
