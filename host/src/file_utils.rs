use serde::Serialize;
use serde_json::{ Value, Error };
use proof_core::eth_utils::{ format_eth_message };
use crate::ethereum::requests::{ ContractRequest, NativeRequest };

use std::fs::{ create_dir_all, File };
use std::io::{ Read, Write };
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
    file.read_to_string(&mut contents).expect("Unable to read the file");
    serde_json::from_str(&contents)
}