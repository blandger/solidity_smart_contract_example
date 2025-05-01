use axum::extract::Path;
use axum::Json;
use axum::routing::{get, MethodRouter};
use common::error::ApiError;
use common::retrieve::RetrieveMessageResponse;
use axum_macros::debug_handler;

#[debug_handler]
pub async fn retrieve_message(
    contract_address: Path<String>,
) -> Result<Json<RetrieveMessageResponse>, ApiError> {
    let contract_address = contract_address.0;
    Ok(Json(RetrieveMessageResponse {
        message: format!("Received contract: {}", contract_address),
        last_updated_block: None,
    }))
}

pub fn retrieve_message_route() -> MethodRouter {
    get(retrieve_message)
}
