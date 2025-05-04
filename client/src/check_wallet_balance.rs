use crate::errors::ClientError;
use crate::load_wallet::load_wallet_from_file;
use alloy::providers::{Provider, ProviderBuilder};
use alloy_primitives::U256;
use crate::TEST_NET_RPC_URL;

pub async fn check_wallet_balance(name: &str) -> Result<U256, ClientError> {
    println!("Check balance signer key: {}", name);
    let wallet = load_wallet_from_file(name)?;
    println!("Wallet address: {}", wallet.address());

    // Connect to Sepolia
    let provider = ProviderBuilder::new().connect(TEST_NET_RPC_URL).await?;

    // Check wallet balance
    let balance = provider.get_balance(wallet.address()).await?;
    println!("Баланс: '{}' wei (ETH)", convert_wei_to_eth(balance));
    Ok(balance)
}

fn convert_wei_to_eth(wei: U256) -> f64 {
    // 1 ETH = 10^18 wei
    let wei_value = wei.to_string();
    let wei_value_u128: u128 = wei_value.parse().unwrap_or(0);

    // Convert ETH to float point number
    let eth_value = wei_value_u128 as f64 / 1_000_000_000_000_000_000.0;

    eth_value
}