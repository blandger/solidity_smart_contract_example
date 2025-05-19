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
    println!("store_message: {:?}", &payload);
    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    let builder = provider
        .send_raw_transaction(tx)
        .await?
        .with_required_confirmations(2)
        .with_timeout(Some(Duration::from_secs(60)));

    let pending_tx = builder.register().await?;

    let tx_hash = *pending_tx.tx_hash();
    println!("store_message tx_hash: {:?}", tx_hash);
    let receipt_tx_hash = pending_tx.await?;
    
    let tx_receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?.ok_or_else(|| ApiError::ReceiptNotFound(receipt_tx_hash))?;
    
    let block_number = tx_receipt.block_number.ok_or_else(|| ApiError::ReceiptBlockNotFound(receipt_tx_hash))?;

    Ok(Json(StoreInContractTransactionResponse {
        transaction_hash: tx_hash.to_string(),
        block_number: Some(block_number),
        status: TransactionStatus::Confirmed,
    }))
}
