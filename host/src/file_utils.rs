use crate::ethereum::requests::{ContractRequest, NativeRequest};
use proof_core::eth_utils::format_eth_message;
use serde::Serialize;
use serde_json::{Error, Value};

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

pub fn parse_json_native(filename: &str) -> Result<NativeRequest, Error> {
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

pub fn parse_json_contract(filename: &str) -> Result<ContractRequest, Error> {
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

fn read_json_file(filename: &str) -> Result<Value, Error> {
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
    use tempfile::tempdir;

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
}
