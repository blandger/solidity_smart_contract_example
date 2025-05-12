use std::error::Error;
use std::f64;
use alloy::eips::Encodable2718;
use alloy::hex;
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy_primitives::{ChainId, U256};
use common::balance::BalanceResponse;
use common::error::ApiError;
use common::transaction_params::TransactionParamsResponse;
use common::transfer::{TransferPayload, TransferTransactionResponse};
use crate::config::BASE_LOCAL_SERVER_URL;
use crate::load_wallet::load_wallet_from_file;

/// Transfer some tokens from one account to another using server side.
pub async fn transfer_amount(account_from: &str, account_to: &str, amount: &str) -> Result<(), Box<dyn Error>> {
    println!("Transfer from '{}' to '{}' the amount '{}'", account_from, account_to, amount);

    // 'account from' Read private key from file
    let account_signer_from = load_wallet_from_file(account_from)?;
    let account_to = load_wallet_from_file(account_to)?;
    let address_from = account_signer_from.address();
    println!("Wallet address from: {}", address_from);

    let address_to = account_to.address();

    let value = amount.parse::<f64>().expect(format!("Amount '{}' to transfer is not parsed !", amount).as_str());
    if value.is_nan() || value.is_sign_negative() {
        return Err("Negative Amount to transfer is not correct !".into());
    }
    
    let amount_wei = U256::from((value * 1e18) as u128);
    println!("Amount to transfer: {} ETH ({} Wei)", value, amount_wei);

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let client = reqwest::Client::new();

    let balance_response = client
        .get(format!("{}/balance/{}", &BASE_LOCAL_SERVER_URL, &address_from))
        .send()
        .await?;

    if !balance_response.status().is_success() {
        let error_text = balance_response.text().await?;
        println!("Failed to get balance for address '{}' because: {}", &address_from, error_text);
        return Err(error_text.into());
    }

    let balance_value: BalanceResponse = balance_response
        .json::<BalanceResponse>()
        .await?;

    let balance_from = balance_value.balance;
    println!("Got balance: '{}' in Wei", balance_from);

    let params_response = client
        .get(format!("{}/tx/{}", &BASE_LOCAL_SERVER_URL, &address_from))
        .send()
        .await?;

    if !params_response.status().is_success() {
        let error_text = params_response.text().await?;
        println!("Failed to get balance for address '{}' because: {}", &address_from, error_text);
        return Err(error_text.into())
    }

    let params_response = params_response.json::<TransactionParamsResponse>()
        .await?;

    let _nonce = U256::try_from(params_response.nonce)?;
    let chain_id = params_response.chain_id;

    let gas_limit = U256::from(21000);
    println!("gas_limit = {gas_limit}");
    let gas_price = U256::from(params_response.gas_price);
    println!("gas_price = {}", &gas_price);
    let gas_fee = gas_price * gas_limit;
    println!("gas_fee = {gas_fee}");

    let total_required = amount_wei
        .checked_add(gas_fee)
        .ok_or("overflow in 'total_required' calculation")?;
    println!("total_required = {total_required}");

    // check if balance is enough for transfer
    if balance_from < total_required {
        return Err(Box::new(ApiError::InsufficientFunds(address_from.to_string(), total_required)));
    }

    // Create transaction for money transfer
    let tx_request = TransactionRequest::default()
        .with_from(address_from)
        .with_to(address_to)
        .with_value(amount_wei)
        .with_nonce(params_response.nonce)
        .with_gas_price(params_response.gas_price)
        .with_chain_id(ChainId::from( chain_id))
        // Set gas limit. For simple ETH transfer 21000 is usually enough
        .with_gas_limit(21000);

    let wallet = EthereumWallet::from(account_signer_from);
    let tx_envelope = tx_request.build(&wallet).await?;

    let serialized_tx = tx_envelope.encoded_2718();
    let signed_tx_hex = hex::encode(serialized_tx.as_slice());

    let transaction_data = TransferPayload {
        address_from: address_from.to_string(),
        address_to: address_to.to_string(),
        amount: value,
        signed_transaction: signed_tx_hex
    };

    let response = client
        .post(format!("{}/transfer", &BASE_LOCAL_SERVER_URL))
        .json(&transaction_data)
        .send()
        .await?;

    if response.status().is_success() {
        let transfer_response = response.json::<TransferTransactionResponse>().await?;

        if let Some(tx_hash) = &transfer_response.transaction_hash {
            println!("Transaction sent successfully!");
            println!("Transaction hash: {}", tx_hash);
        }

        println!("Status: {:?}", &transfer_response.status);
        println!("Block number: {:?}", &transfer_response.block_number);
        println!("Transfer SUCCESS! {:?}", transfer_response.status)
    } else {
        let error_text = response.text().await?;
        println!("Failed to send transaction: {}", error_text);
        return Err(error_text.into())
    }
    Ok(())
}
