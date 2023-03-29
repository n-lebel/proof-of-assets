mod ethereum;
mod file_utils;
mod prover;

use clap::{Arg, Command};
use file_utils::{parse_json_contract, parse_json_native, write_json};
use prover::prove_assets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("prove-assets")
        .version("1.0")
        .author("Nicolas Le Bel")
        .about("CLI tool to process JSON input and execute prove_eth or prove_erc commands")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input JSON file")
                .required(true),
        )
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .value_name("COMMAND")
                .help("Sets the command to execute: prove_eth or prove_erc")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap().as_str();
    let command = matches.get_one::<String>("command").unwrap().as_str();

    match command {
        "prove_eth" => {
            let request = parse_json_native(input_file)?;
            prove_assets(request)?;
        }
        "prove_erc" => {
            let request = parse_json_contract(input_file)?;
            prove_assets(request)?;
        }
        _ => {
            eprintln!("Invalid command. Please use 'prove_eth' or 'prove_erc'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
