use std::error::Error;
use alloy_primitives::hex;
use crate::config::BASE_LOCAL_SERVER_URL;
use crate::load_wallet::load_wallet_from_file;

/// Deploy contract using the specified signer key
pub async fn deploy_contract(signer: &str) -> Result<(), Box<dyn Error>> {
    println!("Deploying contract with signer key: {}", signer);

    let wallet = load_wallet_from_file(signer)?;

    let private_key = wallet.to_bytes().0;
    println!("private_key = {private_key:?}");

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let client = reqwest::Client::new();

    // 2. Sign the deployment transaction (placeholder)
    let signed_transaction = format!("signed_deploy_transaction_with_key_{:?}", private_key.as_slice());

    // 3. Send the transaction to local server
    let response = client.post(format!("{}/deploy", &BASE_LOCAL_SERVER_URL))
        .json(&signed_transaction)
        .send()
        .await?;

    println!("Contract successfully deployed");

    Ok(())
}

fn load_contract_bytecode(path: &str) -> Vec<u8> {
    let hex_string = std::fs::read_to_string(path).expect("Failed to read .bin file");
    hex::decode(hex_string.trim()).expect("Invalid hex in bytecode")
}
