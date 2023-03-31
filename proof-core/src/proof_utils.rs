use crate::eth_utils::{derive_address, recover_public_key};
use eth_trie::{EthTrie, MemoryDB};
use std::sync::Arc;

pub fn verify_signed_message(signature: &Vec<u8>, message: &Vec<u8>, user_address: &[u8]) {
    let pubkey = derive_address(&recover_public_key(signature, message).unwrap()).unwrap();
    if pubkey != user_address {
        panic!("Signature does not match provided address.");
    }
}

pub fn create_eth_trie() -> EthTrie<MemoryDB> {
    let memdb = Arc::new(MemoryDB::new(true));
    let trie = EthTrie::new(memdb.clone());
    trie
}

#[cfg(test)]
mod tests {
    use super::*;
    use eth_trie::Trie;
    use prefix_hex::decode;
    use primitive_types::H256;

    #[test]
    fn test_verify_signed_message_valid_signature() {
        let user_address: [u8; 20] = decode("0x63d90be9ac2859c0b94421281747cefe89b4223c").unwrap();
        let sig = vec![
            123, 16, 121, 69, 94, 249, 166, 247, 223, 86, 238, 232, 204, 63, 99, 205, 220, 19, 23,
            43, 1, 1, 73, 77, 52, 28, 4, 63, 236, 80, 170, 152, 105, 133, 184, 191, 151, 205, 167,
            50, 158, 245, 35, 13, 124, 5, 101, 107, 205, 167, 116, 74, 16, 109, 245, 75, 193, 214,
            190, 174, 183, 144, 242, 164, 0,
        ];
        let msg = "hello world".as_bytes();

        // This test should pass as the signature is valid.
        verify_signed_message(&sig, &msg.to_vec(), &user_address);
    }

    #[test]
    #[should_panic(expected = "Signature does not match provided address.")]
    fn test_verify_signed_message_invalid_signature() {
        let user_address: [u8; 20] = decode("0x63d90be9ac2859c0b94421281747cefe89b4223c").unwrap();
        let sig = vec![
            123, 16, 121, 69, 94, 249, 166, 247, 223, 86, 238, 232, 204, 63, 99, 205, 220, 19, 23,
            43, 1, 1, 73, 77, 52, 28, 4, 63, 236, 80, 170, 152, 105, 133, 184, 191, 151, 205, 167,
            50, 158, 245, 35, 13, 124, 5, 101, 107, 205, 167, 116, 74, 16, 109, 245, 75, 193, 214,
            190, 174, 183, 144, 242, 164, 1,
        ];
        let msg = "hello world".as_bytes();

        // This test should pass as the signature is valid.
        verify_signed_message(&sig, &msg.to_vec(), &user_address);
    }

    #[test]
    fn test_create_eth_trie() {
        let mut trie = create_eth_trie();

        // check that the root hash corresponds to the RLP encoding of an empty list (i.e. the trie is empty)
        assert_eq!(
            trie.root_hash().unwrap(),
            H256::from_slice(
                &decode::<[u8; 32]>(
                    "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
                )
                .unwrap()
            ),
            "Eth trie is not empty"
        )
    }
}
