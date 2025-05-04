mod create;
pub mod load_wallet;
pub mod check_wallet_balance;
pub mod errors;

use clap::{Arg, Command};
use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;
use std::sync::OnceLock;
use tokio;
use reqwest;
use serde_json;
use crate::check_wallet_balance::check_wallet_balance;
use crate::create::create_wallet;
use crate::load_wallet::load_wallet_from_file;

/// Sepolia test net RPC URL
pub const TEST_NET_RPC_URL: &str = "https://rpc-sepolia.rockx.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("wallet-cli")
        .version("0.1.0")
        .author("Your Name")
        .about("wallets and contracts management CLI")
        .subcommand(
            Command::new("create-wallet")
                .about("Creating new wallet")
                .arg(
                    Arg::new("name")
                        .help("Wallet/File name")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("get-balance")
                .about("Check wallet balance")
                .arg(
                    Arg::new("name")
                        .help("Wallet/File name")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("deploy-contract")
                .about("Contract deployment")
                .args(
                    [Arg::new("contract")
                        .help("File name for solidity contract")
                        .required(true)
                        .index(1),
                    Arg::new("signer")
                        .help("File name for private signer key")
                        .required(true)
                        .index(2),
                    ]
                ),
        )
        .subcommand(
            Command::new("store-message")
                .about("Store new data to contract")
                .arg(
                    Arg::new("signer")
                        .help("File name for private signer key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("new_string")
                        .help("New value to store into existing contract")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    init_parent_dir();
    match matches.subcommand() {
        Some(("create-wallet", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            create_wallet(name)?;
        }
        Some(("get-balance", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            check_wallet_balance(name).await?;
        }
        Some(("deploy-contract", sub_matches)) => {
            let signer = sub_matches.get_one::<String>("signer").unwrap();
            deploy_contract(signer).await?;
        }
        Some(("store_message", sub_matches)) => {
            let signer = sub_matches.get_one::<String>("signer").unwrap();
            let new_string = sub_matches.get_one::<String>("new_string").unwrap();
            store_message(signer, new_string).await?;
        }
        _ => {
            println!("Use --help for command information");
        }
    }

    Ok(())
}

/// Deploy contract using the specified signer key
async fn deploy_contract(signer: &str) -> Result<(), Box<dyn Error>> {
    println!("Deploying contract with signer key: {}", signer);

    let wallet = load_wallet_from_file(signer)?;

    let private_key = wallet.to_bytes().0;
    println!("private_key = {private_key:?}");

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    // let client = reqwest::Client::new();

    // 2. Sign the deploy transaction (placeholder)
    // let signed_transaction = format!("signed_deploy_transaction_with_key_{}", private_key);

    // 3. Send the transaction to local server
    // let response = client.post("http://localhost:8000/api/deploy")
    //     .json(&signed_transaction)
    //     .send()
    //     .await?;

    println!("Contract successfully deployed");

    Ok(())
}

/// Store new value in the previously deployed contract
async fn store_message(signer: &str, new_string: &str) -> Result<(), Box<dyn Error>> {
    println!("Storing '{}' in contract with signer key: {}", new_string, signer);

    // Read private key from file
    let private_key_path = format!("{}.private", signer);
    if !Path::new(&private_key_path).exists() {
        return Err(format!("Private key file {} not found", private_key_path).into());
    }

    let private_key = fs::read_to_string(&private_key_path)?;

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let _client = reqwest::Client::new();

    // 2. Create and sign transaction with new value (placeholder)
    let _transaction_data = serde_json::json!({
        "value": new_string,
        "signature": format!("signed_with_key_{}", private_key)
    });

    // 3. Send transaction to local server
    // let response = client.post("http://localhost:8000/api/store")
    //     .json(&transaction_data)
    //     .send()
    //     .await?;

    println!("Value '{}' successfully stored in contract", new_string);

    Ok(())
}

// Global static for parent directory path
static PARENT_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the parent directory based on compile mode
fn init_parent_dir() {
    #[cfg(debug_assertions)]
    {
        // For debug builds (cargo run)
        PARENT_DIR.get_or_init(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "")));
        println!("Running in debug mode, parent directory set to: {:?}", PARENT_DIR.get().unwrap());
    }

    #[cfg(not(debug_assertions))]
    {
        // For release builds
        // Get the executable's directory and use its parent
        PARENT_DIR.get_or_init(|| {
            let exe_path = std::env::current_exe().expect("Failed to get executable path");
            exe_path.parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        });
        println!("Running in release mode, parent directory set to: {:?}", PARENT_DIR.get().unwrap());
    }
}

/// Get path relative to parent directory
fn get_path(relative_path: &str) -> PathBuf {
    let parent = PARENT_DIR.get().expect("Parent directory not initialized");
    parent.join(relative_path)
}
