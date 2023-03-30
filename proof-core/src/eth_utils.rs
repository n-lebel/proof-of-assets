use k256::{
    ecdsa::{recoverable::Signature, signature::Signature as _, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
};
use prefix_hex::decode;
use rlp::{DecoderError, Rlp};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EthGetProofBody {
    pub address: [u8; 20],
    pub account_proof: Vec<Vec<u8>>,
    pub storage_hash: [u8; 32],
    // Can have multiple storage proofs, each one of which is a Vec<Vec<u8>>
    pub storage_proof: Vec<Vec<u8>>,
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

pub fn format_eth_message(message: String) -> String {
    format!(
        "{}{}{}",
        "\x19Ethereum Signed Message:\n",
        message.len(),
        message
    )
}

pub fn recover_public_key(
    sig: &Vec<u8>,
    msg: &Vec<u8>,
) -> Result<VerifyingKey, k256::ecdsa::Error> {
    let signature = Signature::from_bytes(&sig)?;
    println!("{:?}", signature.recover_verifying_key(&msg)?.to_bytes());
    Ok(signature.recover_verifying_key(&msg)?)
}

pub fn derive_address(vk: &VerifyingKey) -> Result<[u8; 20], Box<dyn std::error::Error>> {
    let encoded = vk.to_encoded_point(false);
    let encoded = &encoded.as_bytes()[1..];
    Ok(Keccak256::digest(encoded)[12..].try_into()?)
}

pub fn check_signature(
    sig: &str,
    msg: &str,
    addr: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let pubkey = derive_address(&recover_public_key(&decode(sig).unwrap(), &msg.into()).unwrap())?;

    Ok(pubkey == decode::<[u8; 20]>(addr).unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_decode_ethereum_rlp() {
        let encoded = hex!(
            "e89e208adb2340ea4a803dd312370782adc89080673da3a8841549bca6718ecb8887023bb3fea7fe31"
        );
        let expected = vec![
            vec![
                32, 138, 219, 35, 64, 234, 74, 128, 61, 211, 18, 55, 7, 130, 173, 200, 144, 128,
                103, 61, 163, 168, 132, 21, 73, 188, 166, 113, 142, 203,
            ],
            vec![135, 2, 59, 179, 254, 167, 254, 49],
        ];

        assert_eq!(
            decode_ethereum_rlp(&encoded).unwrap(),
            expected,
            "Decoded RLP is incorrect"
        );
    }

    #[test]
    fn test_format_eth_message() {
        let message = "Hello, Ethereum!".to_string();
        let expected = "\x19Ethereum Signed Message:\n16Hello, Ethereum!";

        assert_eq!(
            format_eth_message(message),
            expected,
            "Formatted message is incorrect"
        );
    }

    #[test]
    fn test_recover_public_key() {
        let sig = vec![
            123, 16, 121, 69, 94, 249, 166, 247, 223, 86, 238, 232, 204, 63, 99, 205, 220, 19, 23,
            43, 1, 1, 73, 77, 52, 28, 4, 63, 236, 80, 170, 152, 105, 133, 184, 191, 151, 205, 167,
            50, 158, 245, 35, 13, 124, 5, 101, 107, 205, 167, 116, 74, 16, 109, 245, 75, 193, 214,
            190, 174, 183, 144, 242, 164, 0,
        ];
        let msg = "hello world".as_bytes().to_vec();

        assert!(
            recover_public_key(&sig, &msg).is_ok(),
            "Public key recovery failed"
        );
    }

    #[test]
    fn test_derive_address() {
        let vk_bytes: [u8; 33] = [
            2, 47, 31, 40, 108, 121, 94, 182, 61, 192, 193, 15, 12, 28, 77, 206, 215, 235, 230, 3,
            248, 61, 137, 230, 107, 228, 145, 246, 5, 200, 189, 26, 13,
        ];

        let vk = VerifyingKey::from_sec1_bytes(&vk_bytes).unwrap();
        let expected_address: [u8; 20] = hex!("2f6c780b5623b98df5a551ed6324d89ab20b0f39");

        assert_eq!(
            derive_address(&vk).unwrap(),
            expected_address,
            "Derived address is incorrect"
        );
    }

    #[test]
    fn test_check_signature() {
        let user_address = "0x63d90be9ac2859c0b94421281747cefe89b4223c";
        let sig = prefix_hex::encode(vec![
            123, 16, 121, 69, 94, 249, 166, 247, 223, 86, 238, 232, 204, 63, 99, 205, 220, 19, 23,
            43, 1, 1, 73, 77, 52, 28, 4, 63, 236, 80, 170, 152, 105, 133, 184, 191, 151, 205, 167,
            50, 158, 245, 35, 13, 124, 5, 101, 107, 205, 167, 116, 74, 16, 109, 245, 75, 193, 214,
            190, 174, 183, 144, 242, 164, 0,
        ]);
        let msg = "hello world";

        assert!(check_signature(&sig, msg, user_address).unwrap());
    }

    #[test]
    fn test_vec_be_bytes_geq() {
        assert!(
            vec_be_bytes_geq(&vec![1, 2, 3], &vec![1, 2, 2]),
            "Comparison is incorrect"
        );
        assert!(
            !vec_be_bytes_geq(&vec![1, 2, 3], &vec![1, 2, 4]),
            "Comparison is incorrect"
        );
        assert!(
            vec_be_bytes_geq(&vec![1, 2, 3], &vec![1, 2, 3]),
            "Comparison is incorrect"
        );
        assert!(
            !vec_be_bytes_geq(&vec![1, 2], &vec![1, 2, 3]),
            "Comparison is incorrect"
        );
        assert!(
            vec_be_bytes_geq(&vec![1, 2, 3], &vec![1, 2]),
            "Comparison is incorrect"
        );
    }

    #[test]
    fn test_be_bytes_geq() {
        assert!(
            be_bytes_geq(&[1, 2, 3], &[1, 2, 2]),
            "Comparison is incorrect"
        );
        assert!(
            !be_bytes_geq(&[1, 2, 3], &[1, 2, 4]),
            "Comparison is incorrect"
        );
        assert!(
            be_bytes_geq(&[1, 2, 3], &[1, 2, 3]),
            "Comparison is incorrect"
        );
        assert!(
            !be_bytes_geq(&[1, 2], &[1, 2, 3]),
            "Comparison is incorrect"
        );
        assert!(be_bytes_geq(&[1, 2, 3], &[1, 2]), "Comparison is incorrect");
    }
}
