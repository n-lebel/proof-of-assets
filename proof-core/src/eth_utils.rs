use k256::{
    ecdsa::{ recoverable::Signature, signature::Signature as _, VerifyingKey },
    elliptic_curve::sec1::ToEncodedPoint,
};
use rlp::{ DecoderError, Rlp };
use serde::{ Deserialize, Serialize };
use sha3::{ Digest, Keccak256 };

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EthGetProofBody {
    pub address: [u8; 20],
    pub account_proof: Vec<Vec<u8>>,
    pub storage_hash: [u8; 32],
    // Can have multiple storage proofs, each one of which is a Vec<Vec<u8>>
    pub storage_proof: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StorageProof {
    pub key: String,
    pub value: String,
    pub proof: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EthGetBlockBody {
    pub number: String,
    pub block_hash: [u8; 32],
    pub storage_hash: [u8; 32],
}

pub fn decode_ethereum_rlp(encoded: &[u8]) -> Result<Vec<Vec<u8>>, DecoderError> {
    let rlp = Rlp::new(encoded);
    let decoded: Vec<Vec<u8>> = rlp.as_list()?;
    Ok(decoded)
}

pub fn recover_public_key(
    sig: &Vec<u8>,
    msg: &Vec<u8>
) -> Result<VerifyingKey, k256::ecdsa::Error> {
    let signature = Signature::from_bytes(&sig)?;
    Ok(signature.recover_verifying_key(&msg)?)
}

pub fn derive_address(vk: &VerifyingKey) -> Result<[u8; 20], Box<dyn std::error::Error>> {
    let encoded = vk.to_encoded_point(false);
    let encoded = &encoded.as_bytes()[1..];
    Ok(Keccak256::digest(encoded)[12..].try_into()?)
}

pub fn vec_be_bytes_geq(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    if a.len() > b.len() {
        true
    } else if a.len() < b.len() {
        false
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                return true;
            } else if a[i] < b[i] {
                return false;
            }
        }
        true
    }
}

pub fn be_bytes_geq(a: &[u8], b: &[u8]) -> bool {
    if a.len() > b.len() {
        true
    } else if a.len() < b.len() {
        false
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                return true;
            } else if a[i] < b[i] {
                return false;
            }
        }
        true
    }
}