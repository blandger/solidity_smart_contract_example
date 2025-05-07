use std::error::Error;
use std::f64;
use alloy_primitives::{Address, U256};
use crate::load_wallet::load_wallet_from_file;

/// Transfer some tokens from one account to another using server side.
pub async fn transfer_amount(account_from: &str, account_to: &str, amount: &str) -> Result<(), Box<dyn Error>> {
    println!("Transfer from '{}' to '{}' the amount '{}'", account_from, account_to, amount);

    // 'account from' Read private key from file
    let account_signer_from = load_wallet_from_file(account_from)?;
    let address = account_signer_from.address();
    println!("Wallet address from: {}", address);

    let address_to = Address::parse_checksummed(account_to, None).expect(format!("Account address '{}' to transfer to is not correct !", account_to).as_str());

    let value = amount.parse::<f64>().expect(format!("Amount '{}' to transfer is not correct !", amount).as_str());
    let amount_wei = U256::from((value * 1e18) as u128);
    println!("Amount to transfer: {} ETH ({} Wei)", value, amount_wei);


    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let _client = reqwest::Client::new();

    // 2. Create and sign transaction with new value (placeholder)
    /*let _transaction_data = serde_json::json!({
        "value": new_string,
        "signature": format!("signed_with_key_{}", private_key)
    });

    // 3. Send transaction to local server
    let response = client.post("http://localhost:8000/api/store")
        .json(&transaction_data)
        .send()
        .await?;

    println!("Value '{}' successfully stored in contract", new_string);*/

    Ok(())
}