use crate::ethereum::requests::{ContractRequest, NativeRequest};
use proof_core::eth_utils::format_eth_message;
use risc0_zkvm::Receipt;
use serde::{de::Error, Serialize};
use serde_json::{Error as SerdeJsonError, Value};

use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn write_json<T: Serialize>(value: &T, file_path: &str) -> std::io::Result<()> {
    // Serialize the struct to JSON
    let json = serde_json::to_string(value).unwrap();

    // Create the path if it doesn't exist
    let path = Path::new(file_path);
    create_dir_all(&path)?;

    // Append "receipt.json" to the path
    let file_path_with_receipt = path.join("receipt.json");

    // Write the JSON to the file
    let mut file = File::create(file_path_with_receipt)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn parse_json_native(filename: &str) -> Result<NativeRequest, SerdeJsonError> {
    let data = read_json_file(filename).unwrap();

    let provider = String::from(data["provider"].as_str().unwrap());
    let user_address = String::from(data["user_address"].as_str().unwrap());
    let block_number = String::from(data["block_number"].as_str().unwrap());
    let signature = String::from(data["signature"].as_str().unwrap());
    let message = format_eth_message(String::from(data["message"].as_str().unwrap()));
    let expected_balance = data["expected_balance"].as_u64().unwrap();

    Ok(NativeRequest {
        provider,
        user_address,
        block_number,
        signature,
        message,
        expected_balance,
    })
}

pub fn parse_json_contract(filename: &str) -> Result<ContractRequest, SerdeJsonError> {
    let data = read_json_file(filename)?;

    let provider = String::from(data["provider"].as_str().unwrap());
    let user_address = String::from(data["user_address"].as_str().unwrap());
    let block_number = String::from(data["block_number"].as_str().unwrap());
    let signature = String::from(data["signature"].as_str().unwrap());
    let message = format_eth_message(String::from(data["message"].as_str().unwrap()));
    let expected_balance = data["expected_balance"].as_u64().unwrap();

    let contract_address = String::from(data["contract_address"].as_str().unwrap());
    let balance_slot = String::from(data["balance_slot"].as_str().unwrap());

    Ok(ContractRequest {
        provider,
        user_address,
        block_number,
        signature,
        message,
        expected_balance,
        contract_address,
        balance_slot,
    })
}

pub fn parse_json_receipt(path: &str) -> Result<Receipt, SerdeJsonError> {
    let data = read_json_file(path)?;

    // Unwrap seal into Vec<Value>
    let seal = data["seal"]
        .as_array()
        .ok_or(SerdeJsonError::missing_field("seal"))?;
    // Convert values into u32
    let seal: Result<Vec<u32>, SerdeJsonError> = seal
        .iter()
        .map(|v| {
            v.as_u64()
                .ok_or(SerdeJsonError::custom("Invalid type in seal array"))
                .map(|num| num as u32)
        })
        .collect();

    // Unwrap journal into Vec<Value>
    let journal = data["journal"]
        .as_array()
        .ok_or(SerdeJsonError::missing_field("journal"))?;
    // Convert values into u8
    let journal: Result<Vec<u8>, SerdeJsonError> = journal
        .iter()
        .map(|v| {
            v.as_u64()
                .ok_or(SerdeJsonError::custom("Invalid type in journal array"))
                .map(|num| num as u8)
        })
        .collect();

    Ok(Receipt::new(&journal?, &seal?))
}

fn read_json_file(filename: &str) -> Result<Value, SerdeJsonError> {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    serde_json::from_str(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_json_contract, parse_json_native, write_json};
    use proof_core::eth_utils::format_eth_message;
    use serde_json::json;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_write_json() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().to_str().unwrap();

        let data = json!({
            "key": "value"
        });

        write_json(&data, file_path).unwrap();
        let path = Path::new(file_path).join("receipt.json");
        assert!(path.exists());
    }

    #[test]
    fn test_read_json_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_read.json");
        let test_data = r#"
        {
            "key": "value"
        }
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();
        let contents = read_json_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(contents["key"], "value");
    }

    #[test]
    fn test_parse_json_native() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_parse_native.json");
        let test_data = r#"
        {
            "provider": "test_provider",
            "user_address": "test_user_address",
            "block_number": "12345",
            "signature": "test_signature",
            "message": "test_message",
            "expected_balance": 1000
        }
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();
        let native_request = parse_json_native(file_path.to_str().unwrap()).unwrap();

        assert_eq!(native_request.provider, "test_provider");
        assert_eq!(native_request.user_address, "test_user_address");
        assert_eq!(native_request.block_number, "12345");
        assert_eq!(native_request.signature, "test_signature");
        assert_eq!(
            native_request.message,
            format_eth_message("test_message".to_string())
        );
        assert_eq!(native_request.expected_balance, 1000);
    }

    #[test]
    fn test_parse_json_contract() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_parse_contract.json");
        let test_data = r#"
        {
            "provider": "test_provider",
            "user_address": "test_user_address",
            "block_number": "12345",
            "signature": "test_signature",
            "message": "test_message",
            "expected_balance": 1000,
            "contract_address": "test_contract_address",
            "balance_slot": "test_balance_slot"
        }
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();
        let contract_request = parse_json_contract(file_path.to_str().unwrap()).unwrap();

        assert_eq!(contract_request.provider, "test_provider");
        assert_eq!(contract_request.user_address, "test_user_address");
        assert_eq!(contract_request.block_number, "12345");
        assert_eq!(contract_request.signature, "test_signature");
        assert_eq!(
            contract_request.message,
            format_eth_message("test_message".to_string())
        );
        assert_eq!(contract_request.expected_balance, 1000);
        assert_eq!(contract_request.contract_address, "test_contract_address");
        assert_eq!(contract_request.balance_slot, "test_balance_slot");
    }

    fn create_temp_json_file(json_content: &serde_json::Value) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().expect("Unable to create temporary file");
        let content = json_content.to_string();
        temp_file
            .write_all(content.as_bytes())
            .expect("Unable to write to temporary file");
        temp_file
    }

    #[test]
    fn test_parse_json_receipt_valid() {
        let valid_json = json!({
            "seal": [1, 2, 3],
            "journal": [42, 34, 12]
        });

        let temp_file = create_temp_json_file(&valid_json);
        let path = temp_file.path().to_str().unwrap();
        let receipt = parse_json_receipt(path).unwrap();

        assert_eq!(receipt.seal, vec![1, 2, 3]);
        assert_eq!(receipt.journal, vec![42, 34, 12]);
    }

    #[test]
    fn test_parse_json_receipt_missing_seal() {
        let missing_seal_json = json!({
            "journal": [42, 34, 12]
        });

        let temp_file = create_temp_json_file(&missing_seal_json);
        let path = temp_file.path().to_str().unwrap();
        let result = parse_json_receipt(path);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_json_receipt_missing_journal() {
        let missing_journal_json = json!({
            "seal": [1, 2, 3]
        });

        let temp_file = create_temp_json_file(&missing_journal_json);
        let path = temp_file.path().to_str().unwrap();
        let result = parse_json_receipt(path);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_json_receipt_invalid_type() {
        let invalid_type_json = json!({
            "seal": [1, 2, "invalid"],
            "journal": [42, 34, 12]
        });

        let temp_file = create_temp_json_file(&invalid_type_json);
        let path = temp_file.path().to_str().unwrap();
        let result = parse_json_receipt(path);

        assert!(result.is_err());
        // The test will panic when unwrapping invalid data, so we won't check the specific error
    }
}
