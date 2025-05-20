use std::time::Duration;
use alloy::hex;
use alloy::providers::Provider;
use axum::extract::State;
use axum::Json;
use common::error::ApiError;
use common::store::{StoreMessagePayload, StoreInContractTransactionResponse, TransactionStatus};
use crate::state::AppState;

// Store message in contract
pub async fn store_message(
    State(state): State<AppState>,
    Json(payload): Json<StoreMessagePayload>,
) -> Result<Json<StoreInContractTransactionResponse>, ApiError> {
    println!("Called store_message:\n{:?}", &payload);
    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    println!("Sending store_message signed tx....");
    let builder = provider
        .send_raw_transaction(tx)
        .await?
        .with_required_confirmations(2)
        .with_timeout(Some(Duration::from_secs(60)));

    println!("Pending tx....");
    let pending_tx = builder.register().await?;
    let tx_hash = *pending_tx.tx_hash();
    println!("Done store_message tx_hash: {:?}", tx_hash);

    Ok(Json(StoreInContractTransactionResponse {
        transaction_hash: Some(tx_hash.to_string()),
        block_number: None,
        status: TransactionStatus::Confirmed,
    }))
}
