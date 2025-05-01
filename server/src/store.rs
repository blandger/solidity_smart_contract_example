use axum::Json;
use common::error::ApiError;
use common::store::{StoreMessagePayload, TransactionResponse};

// Store message in contract
pub async fn store_message(
    Json(payload): Json<StoreMessagePayload>,
) -> Result<Json<TransactionResponse>, ApiError> {
    todo!()
}
