use crate::config::BASE_LOCAL_SERVER_URL;
use crate::errors::ClientError;
use crate::load_wallet::load_wallet_from_file;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::Address;
use common::config::init_test_net_url;
use common::contract::MessageStorageContract;
use common::store::{StoreInContractTransactionResponse, StoreMessagePayload};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

/// Store new value in the previously deployed contract
pub async fn store_message(
    signer: &str,
    contract_address: &str,
    new_message_value: &str,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Storing new value '{}' in contract address '{}' using signer key: {}",
        new_message_value, contract_address, signer
    );

    // Read private key from file
    let account_signer_from = load_wallet_from_file(signer)?;
    // create transaction to set new message value in contract (by address)
    let signed_tx_hex = create_contract_store_message_transaction_locally(
        contract_address,
        account_signer_from.clone(),
        new_message_value,
    )
    .await?;
    println!("Tx sender for store message: {:?}", &account_signer_from.address());

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(6000)) // 10 minutes
        .build()
        .expect("Failed to build reqwest client");

    // 2. Send signed transaction with new value
    let signed_transaction = StoreMessagePayload {
        signed_transaction: signed_tx_hex,
    };

    println!("Sending tx by owner: '{}'", &signer);
    // 3. Send transaction to local server
    let response = client
        .post(format!("{}/store-message", &BASE_LOCAL_SERVER_URL))
        .json(&signed_transaction)
        .send()
        .await?;
    println!("Got response: {:?}", &response);

    if response.status().is_success() {
        let store_response = response
            .json::<StoreInContractTransactionResponse>()
            .await?;

        if let Some(tx_hash) = &store_response.transaction_hash {
            println!("Deploy Transaction sent successfully!");
            println!("Deploy Transaction hash: {}", tx_hash);
        }

        println!("Status: {:?}", &store_response.status);
        println!("Block number: {:?}", &store_response.block_number);
        println!(
            "New Value '{}' successfully stored in contract by address: {}",
            new_message_value, contract_address
        );
    } else {
        let error_text = response.text().await?;
        println!(
            "Failed to send contract deployment transaction: {}",
            error_text
        );
        return Err(error_text.into());
    }

    Ok(())
}

async fn create_contract_store_message_transaction_locally(
    contract_address: &str,
    sign_contract_owner: PrivateKeySigner,
    new_contract_message_value: &str,
) -> Result<String, ClientError> {
    let address = Address::from_str(contract_address)?;

    // We create new local Provider here
    let rpc_url = init_test_net_url();
    // Connect to Sepolia locally on CLI
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    // We should use local Provider because it's impossible create ABI contract without it for creation TX for a calling 'store message' operation
    let contract = MessageStorageContract::new(address, Arc::new(provider))?;
    // Create TX for 'store message' operation
    let tx_call_contract = contract
        .store_message_hex(sign_contract_owner, new_contract_message_value)
        .await?;
    println!("Created tx to call a contract");
    Ok(tx_call_contract)
}
