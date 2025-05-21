use crate::state::AppState;
use alloy::hex;
use alloy::providers::Provider;
use alloy_primitives::private::rand;
use axum::Json;
use axum::extract::State;
use axum_macros::debug_handler;
use common::deploy::{DeployContractPayload, DeployContractResponse};
use common::error::ApiError;
use common::store::TransactionStatus;
use rand::Rng;
use std::time::Duration;
use tracing::{debug, info, warn};

#[debug_handler]
pub async fn deploy_contract(
    State(state): State<AppState>,
    Json(payload): Json<DeployContractPayload>,
) -> Result<Json<DeployContractResponse>, ApiError> {
    info!("deploy_contract... address: {}", &payload.address_from);

    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    debug!("Sending smart contract deploy tx ...");
    // Send the transaction, but do not wait for confirmations here
    let pending_tx_hash = provider.send_raw_transaction(tx).await?;

    let tx_hash = pending_tx_hash.tx_hash();
    debug!("Got transaction hash: {}", tx_hash);

    // Optionally - wait for the transaction to be sent to the network
    // but do not wait for block confirmations
    let mut attempts = 0;
    let max_attempts = 10;
    let mut receipt = None;

    // Add a limited number of attempts to get a receipt
    while attempts < max_attempts {
        attempts += 1;
        debug!("Attempt {} to get receipt", attempts);
        let jitter = {
            let mut rng = rand::rng();
            rng.random_range(1..9)
        };
        debug!("Generated jitter: {}", jitter);

        match provider.get_transaction_receipt(*tx_hash).await {
            Ok(Some(r)) => {
                receipt = Some(r);
                break;
            }
            Ok(None) => {
                debug!("Receipt not yet available, waiting...");
                tokio::time::sleep(Duration::from_secs(jitter * attempts)).await;
            }
            Err(e) => {
                debug!("Error getting receipt: {:?}", e);
                tokio::time::sleep(Duration::from_secs(jitter * attempts)).await;
            }
        }
    }

    // Even if we didn't get a receipt, return success with the transaction hash
    match receipt {
        Some(r) => {
            let contract_address = r
                .contract_address
                .ok_or_else(|| ApiError::ReceiptContractAddressNotFound(*tx_hash))?;

            info!(
                "SUCCESS! Contract is deployed at address '{}'",
                contract_address
            );

            Ok(Json(DeployContractResponse {
                transaction_hash: Some(tx_hash.to_string()),
                contract_address: contract_address.to_string(),
                block_number: Some(r.block_number.unwrap()),
                status: TransactionStatus::Confirmed,
            }))
        }
        None => {
            warn!("Transaction sent but receipt not yet available");
            // Return a successful response with the transaction hash, but without full data
            Ok(Json(DeployContractResponse {
                transaction_hash: Some(tx_hash.to_string()),
                contract_address: String::from("Pending"), // Not successful probably
                block_number: None,
                status: TransactionStatus::Pending,
            }))
        }
    }
}
