use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{MethodRouter, get};
use axum_macros::debug_handler;
use common::error::ApiError;
use common::retrieve::RetrieveMessageResponse;

#[debug_handler]
pub async fn retrieve_message(
    Path(contract_address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<RetrieveMessageResponse>, ApiError> {
    Ok(Json(RetrieveMessageResponse {
        message: format!("Received contract: {}", contract_address),
        last_updated_block: None,
    }))
}

pub fn retrieve_message_route() -> MethodRouter<AppState> {
    get(retrieve_message)
}
