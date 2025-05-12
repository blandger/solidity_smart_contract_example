use alloy_primitives::Address;
use axum::extract::State;
use axum::Json;
use common::deploy::{DeployContractPayload, DeployContractResponse};
use common::error::ApiError;
use crate::state::AppState;

pub async fn deploy_contract(
    State(state): State<AppState>,
    Json(payload): Json<DeployContractPayload>,
) -> Result<Json<DeployContractResponse>, ApiError> {
    println!("deploy_contract... address: {}", &payload.address_from);
    let address = Address::parse_checksummed(&payload.address_from, None)?;

    let provider = state.provider;
    
    todo!()
}
