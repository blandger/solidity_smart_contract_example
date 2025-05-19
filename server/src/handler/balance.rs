use alloy::providers::Provider;
use alloy_primitives::{Address, U256};
use axum::extract::{Path, State};
use axum::Json;
use axum::routing::{get, MethodRouter};
use common::balance::BalanceResponse;
use common::error::ApiError;
use crate::state::AppState;

pub async fn get_balance(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, ApiError> {
    println!("get_balance_route... address: {}", &address);
    let address = Address::parse_checksummed(&address, None)?;
    let balance = state.provider.get_balance(address).await?;
    let eth = convert_wei_to_eth(balance);
    println!("Got balance for address: '{}' = '{}' wei ({} eth)", &address, &balance, &eth);
    Ok(Json(BalanceResponse {
        balance,
    }))
}

fn convert_wei_to_eth(wei: U256) -> f64 {
    // 1 ETH = 10^18 wei
    let wei_value = wei.to_string();
    let wei_value_u128: u128 = wei_value.parse().unwrap_or(0);

    // Convert ETH to float point number
    let eth_value = wei_value_u128 as f64 / 1_000_000_000_000_000_000.0;

    eth_value
}

pub fn get_balance_route() -> MethodRouter<AppState> {
    get(get_balance)
}
