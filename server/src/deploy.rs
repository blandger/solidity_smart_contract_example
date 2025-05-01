use axum::Json;
use common::deploy::{DeployContractPayload, DeployContractResponse};
use common::error::ApiError;

pub async fn deploy_contract(
    Json(payload): Json<DeployContractPayload>,
) -> Result<Json<DeployContractResponse>, ApiError> {
    todo!()
}
