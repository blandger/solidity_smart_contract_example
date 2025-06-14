use std::error::Error;
use std::path::Path;
use alloy::eips::Encodable2718;
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy_primitives::{hex, U256};
use tracing::{debug, error, info};
use common::deploy::{DeployContractPayload, DeployContractResponse};
use crate::config::{APPROXIMATE_CONTRACT_DEPLOY_GAS_PRICE, BASE_LOCAL_SERVER_URL};
use crate::errors::ClientError;
use crate::init::get_path;
use crate::load_wallet::load_wallet_from_file;
use crate::transfer::check_account_balance;

/// Deploy contract using the specified signer key
pub async fn deploy_contract(contract: &str, signer: &str) -> Result<(), Box<dyn Error>> {
    info!("Deploying contract '{}' by signer key: {}", &signer, &contract);

    let account_signer_from = load_wallet_from_file(signer)?;

    // let private_key = account_signer_from.to_bytes().0;
    debug!("Reading contract binary file: {contract}");
    let contract_byte_code = load_contract_bytecode(contract)?;
    debug!("Done reading contract file: {contract}");

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(6000)) // 10 minutes
        .build()?;

    let contract_gas_limit = U256::from(APPROXIMATE_CONTRACT_DEPLOY_GAS_PRICE);

    // Check balance
    debug!("Checking balance before deploying contract: '{}' (wei)", &APPROXIMATE_CONTRACT_DEPLOY_GAS_PRICE);
    let params_response = check_account_balance(&account_signer_from.address(), U256::from(0), contract_gas_limit, &client).await?;

    debug!("Start contract deployment by owner: '{}'", &account_signer_from.address());
    let tx_request = TransactionRequest::default()
        .with_from(account_signer_from.address())
        .with_nonce(params_response.nonce)
        .with_gas_price(params_response.gas_price)
        .with_gas_limit(APPROXIMATE_CONTRACT_DEPLOY_GAS_PRICE)
        .with_chain_id(params_response.chain_id)
        .with_deploy_code(contract_byte_code);

    let wallet = EthereumWallet::from(account_signer_from.clone());
    let tx_envelope = tx_request.build(&wallet).await?;

    let serialized_tx = tx_envelope.encoded_2718();
    let signed_tx_hex = hex::encode(serialized_tx.as_slice());

    // 2. Sign the deployment transaction (placeholder)
    let signed_transaction = DeployContractPayload {
        address_from: account_signer_from.address().to_string(),
        signed_transaction: signed_tx_hex,
    };
    debug!("Sending tx by owner: '{}'", &account_signer_from.address());

    // 3. Send the transaction to local server
    let response = client.post(format!("{}/deploy-contract", &BASE_LOCAL_SERVER_URL))
        .json(&signed_transaction)
        .send()
        .await?;
    debug!("Got response: {:?}", &response);

    if response.status().is_success() {
        let deploy_response = response.json::<DeployContractResponse>().await?;

        if let Some(tx_hash) = &deploy_response.transaction_hash {
            debug!("Store Massage Transaction sent successfully!");
            info!("Transaction hash: {}", tx_hash);
        }
        debug!("Status: {:?}", &deploy_response.status);
        debug!("Block number: {:?}", &deploy_response.block_number);
        info!("Contract deployment SUCCESS! Contract address: {:?}", deploy_response.contract_address)
    } else {
        let error_text = response.text().await?;
        error!("Failed to send 'new message tx' to contract: {}", error_text);
        return Err(error_text.into())
    }

    Ok(())
}

fn load_contract_bytecode(contract_file_name: &str) -> Result<Vec<u8>, ClientError> {
    let contract_file_path = get_path(&format!("{}.bin", contract_file_name));
    if !Path::new(&contract_file_path).exists() {
        return Err(ClientError::NotFoundContractFile(contract_file_name.to_string(), contract_file_path));
    }
    let hex_string = std::fs::read_to_string(contract_file_path.clone()).map_err(|err| {
        error!("Error reading contract file: {}", err);
        ClientError::ReadContractFile(contract_file_name.to_string(), contract_file_path.clone())
    })?;
    let hex_string = hex_string.trim().trim_start_matches("0x");

    let bytecode = hex::decode(hex_string)?; // Convert hex string to Vec<u8>
    Ok(bytecode)
}
