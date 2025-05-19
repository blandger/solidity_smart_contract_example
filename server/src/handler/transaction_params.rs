use alloy::providers::Provider;
use alloy_primitives::Address;
use axum::extract::{Path, State};
use axum::Json;
use axum::routing::{get, MethodRouter};
use common::error::ApiError;
use common::transaction_params::TransactionParamsResponse;
use crate::state::AppState;

pub async fn get_transaction_params(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<TransactionParamsResponse>, ApiError> {
    println!("get_transaction_params... {:?}", &address);
    let provider = state.provider;

    let address_value = Address::parse_checksummed(&address, None)?;
    // Get nonce and gas_price
    let nonce = provider.get_transaction_count(address_value).await?;
    let gas_price = provider.get_gas_price().await?;
    let chain_id = provider.get_chain_id().await?;
    println!("get_transaction_params... nonce: {}, gas_price: {}, chain_id: {}", nonce, gas_price, chain_id);

    // Return the parameters to the client
    Ok(Json(TransactionParamsResponse {
        nonce,
        gas_price,
        chain_id, // Sepolia
    }))
}

pub fn get_transaction_params_route() -> MethodRouter<AppState> {
    get(get_transaction_params)
}
