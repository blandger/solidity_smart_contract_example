use axum::extract::State;
use axum::Json;
use common::error::ApiError;
use common::store::{StoreMessagePayload, TransactionResponse};
use crate::state::AppState;

// Store message in contract
pub async fn store_message(
    State(state): State<AppState>,
    Json(payload): Json<StoreMessagePayload>,
) -> Result<Json<TransactionResponse>, ApiError> {
    todo!()
}
