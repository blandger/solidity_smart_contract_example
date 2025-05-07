use axum::extract::State;
use axum::Json;
use common::deploy::{DeployContractPayload, DeployContractResponse};
use common::error::ApiError;
use crate::state::AppState;

pub async fn deploy_contract(
    State(state): State<AppState>,
    Json(payload): Json<DeployContractPayload>,
) -> Result<Json<DeployContractResponse>, ApiError> {
    todo!()
}
