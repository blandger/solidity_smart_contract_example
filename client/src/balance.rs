use alloy_primitives::U256;
use crate::errors::ClientError;
use crate::load_wallet::load_wallet_from_file;
use common::balance::BalanceResponse;
use crate::config::BASE_LOCAL_SERVER_URL;
use crate::errors::ClientError::Server;

pub async fn check_wallet_balance(name: &str) -> Result<f64, ClientError> {
    println!("Check balance signer key: {}", name);
    let wallet = load_wallet_from_file(name)?;
    let address = wallet.address();
    println!("Wallet address: {}", address);

    // 1. Create an HTTP client
    let client = reqwest::Client::new();

    let balance_response = client
        .get(format!("{}/balance/{}", &BASE_LOCAL_SERVER_URL, &address))
        .send()
        .await?;

    if !balance_response.status().is_success() {
        let error_text = balance_response.text().await?;
        println!("Failed to get balance for address '{}' because: {}", &address, error_text);
        return Err(Server(error_text));
    }

    let balance: BalanceResponse = balance_response
        .json::<BalanceResponse>()
        .await?;
    let wei = convert_wei_to_eth(balance.balance);
    println!("Balance: '{:?}' wei (ETH)", &wei);
    Ok(wei)
}

fn convert_wei_to_eth(wei: U256) -> f64 {
    // 1 ETH = 10^18 wei
    let wei_value = wei.to_string();
    let wei_value_u128: u128 = wei_value.parse().unwrap_or(0);

    // Convert ETH to float point number
    let eth_value = wei_value_u128 as f64 / 1_000_000_000_000_000_000.0;

    eth_value
}