use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
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
