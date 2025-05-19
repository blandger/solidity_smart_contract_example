use std::sync::Arc;
use alloy_primitives::Address;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{MethodRouter, get};
use axum_macros::debug_handler;
use common::contract::MessageStorageContract;
use common::error::ApiError;
use common::retrieve::RetrieveMessageResponse;

#[debug_handler]
pub async fn retrieve_message(
    State(state): State<AppState>,
    Path(contract_address): Path<String>,
) -> Result<Json<RetrieveMessageResponse>, ApiError> {
    println!("Retrieving message from contract deployed by address: {}", &contract_address);
    let contract_address = contract_address.parse::<Address>()?;

    let provider = state.provider;

    let contract = MessageStorageContract::new(contract_address, Arc::clone(&provider))?;
    let message_value = contract.retrieve_message().await?;
    println!("Got message: {}", &message_value);
    
    Ok(Json(RetrieveMessageResponse {
        message: message_value,
        last_updated_block: None,
    }))
}

pub fn retrieve_message_route() -> MethodRouter<AppState> {
    get(retrieve_message)
}
