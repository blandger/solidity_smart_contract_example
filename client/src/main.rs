pub mod balance;
mod config;
mod create;
pub mod deploy;
pub mod errors;
pub mod load_wallet;
pub mod read;
pub mod store;
pub mod transfer;

use crate::balance::check_wallet_balance;
use crate::create::create_wallet;
use crate::deploy::deploy_contract;
use crate::read::read_message;
use crate::store::store_message;
use crate::transfer::transfer_amount;
use clap::{Arg, Command};
use std::error::Error;
use std::path::PathBuf;
use std::sync::OnceLock;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    if let Err(e) = run_app().await {
        eprintln!("Error: {}", e); // Use {:?}
        std::process::exit(1);
    }
}

async fn run_app() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("wallet-cli")
        .version("0.1.0")
        .author("Yuriy")
        .about("Test Ethereum wallets and contracts management CLI")
        .subcommand(
            Command::new("create-wallet")
                .about("Creating new wallet")
                .arg(
                    Arg::new("name")
                        .help("Wallet's file name")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("get-balance")
                .about("Check wallet balance")
                .arg(
                    Arg::new("name")
                        .help("Wallet's file name")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("transfer")
                .about("Transfer X money from one to another wallet/account")
                .args([
                    Arg::new("from")
                        .help("Local file name for private key of Account to transfer money from")
                        .required(true)
                        .index(1),
                    Arg::new("to")
                        .help("Account Address to transfer money to")
                        .required(true)
                        .index(2),
                    Arg::new("amount")
                        .help("Amount to transfer")
                        .required(true)
                        .index(3),
                ]),
        )
        .subcommand(
            Command::new("deploy-contract")
                .about("Contract deployment")
                .args([
                    Arg::new("signer")
                        .help("File name for private signer key")
                        .required(true)
                        .index(1),
                    Arg::new("contract")
                        .help("File name for local solidity contract (.bin)")
                        .required(true)
                        .index(2),
                ]),
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
                    Arg::new("contract-address")
                        .help("Address of existing contract")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("new-value")
                        .help("New value to store into existing contract")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            Command::new("read-message")
                .about("Read data from deployed contract")
                .arg(
                    Arg::new("contact-hash")
                        .help("Contract address hash")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    init_parent_dir();
    match matches.subcommand() {
        // Generates two keys and writes them to files using name provided
        Some(("create-wallet", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            create_wallet(name)?;
        }
        Some(("get-balance", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            check_wallet_balance(name).await?;
            // check_wallet_balance_local_provider(name).await?;
        }
        Some(("transfer", sub_matches)) => {
            // that is a local file name !!
            let account_from = sub_matches.get_one::<String>("from").unwrap();
            // that is an Ethereum address !!
            let account_to = sub_matches.get_one::<String>("to").unwrap();
            // that is an amount number as string
            let amount = sub_matches.get_one::<String>("amount").unwrap();
            transfer_amount(account_from, account_to, amount).await?;
        }
        Some(("deploy-contract", sub_matches)) => {
            let contract = sub_matches.get_one::<String>("contract").unwrap();
            let signer = sub_matches.get_one::<String>("signer").unwrap();
            deploy_contract(contract, signer).await?;
        }
        Some(("store-message", sub_matches)) => {
            // name of the file with private key for contract owner
            let signer = sub_matches.get_one::<String>("signer").unwrap();
            // deployed contract address
            let contract_address = sub_matches.get_one::<String>("contract-address").unwrap();
            // new value inside "", like = "Hello world"
            let new_message = sub_matches.get_one::<String>("new-value").unwrap();
            store_message(signer, contract_address, new_message).await?;
        }
        Some(("read-message", sub_matches)) => {
            let contact_hash = sub_matches.get_one::<String>("contact-hash").unwrap();
            read_message(contact_hash).await?;
        }
        _ => {
            println!("Use --help for command information");
        }
    }

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
        println!(
            "Running in debug mode, parent directory set to: {:?}",
            PARENT_DIR.get().unwrap()
        );
    }

    #[cfg(not(debug_assertions))]
    {
        use std::path::Path;
        // For release builds
        // Get the executable's directory and use its parent
        PARENT_DIR.get_or_init(|| {
            let exe_path = std::env::current_exe().expect("Failed to get executable path");
            exe_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        });
        println!(
            "Running in release mode, parent directory set to: {:?}",
            PARENT_DIR.get().unwrap()
        );
    }
}

/// Get path relative to parent directory
fn get_path(relative_path: &str) -> PathBuf {
    let parent = PARENT_DIR.get().expect("Parent directory is not initialized");
    parent.join(relative_path)
}
