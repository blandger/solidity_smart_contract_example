use std::str::FromStr;
use alloy::providers::{Provider, ProviderBuilder};
use alloy_primitives::{Address, U256};
use crate::errors::ClientError;
use crate::load_wallet::recipient_address_from_string_or_local_file;
use common::balance::BalanceResponse;
use common::config::init_test_net_url;
use crate::config::BASE_LOCAL_SERVER_URL;
use crate::errors::ClientError::Server;

pub async fn check_wallet_balance(name: &str) -> Result<f64, ClientError> {
    println!("Check balance by public address as string (OR by local private signer key file): {}", name);
    let address = recipient_address_from_string_or_local_file(name)?;
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
    let wei = balance.balance;
    println!("Balance: '{:?}' wei", &wei);
    let eth = convert_wei_to_eth(balance.balance);
    Ok(eth)
}

pub(crate) fn convert_wei_to_eth(wei: U256) -> f64 {
    // 1 ETH = 10^18 wei
    let wei_value = wei.to_string();
    let wei_value_u128: u128 = wei_value.parse().unwrap_or(0);

    // Convert ETH to float point number
    let eth_value = wei_value_u128 as f64 / 1_000_000_000_000_000_000.0;

    eth_value
}

pub async fn check_wallet_balance_local_provider(name: &str) -> Result<f64, ClientError> {

    let address = Address::from_str(name)?;

    let rpc_url = init_test_net_url();
    // Connect to Sepolia
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    // Check wallet balance
    let balance = provider.get_balance(address).await?;
    // let balance = provider.get_balance_with_block(address, BlockId::Number(BlockNumberOrTag::Latest)).await?;

    println!("Balance: '{}' = '{}' wei ( {} ETH)", &address, &balance, convert_wei_to_eth(balance));
    let eth = convert_wei_to_eth(balance);
    Ok(eth)
}