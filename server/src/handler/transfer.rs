use crate::state::AppState;
use alloy::hex;
use axum::extract::State;
use common::error::ApiError;
use common::store::TransactionStatus;
use common::transfer::{TransactionResponse, TransferPayload};

pub async fn transfer(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<TransferPayload>,
) -> Result<axum::Json<TransactionResponse>, ApiError> {
    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    // Send a transaction, and configure the pending transaction.
    let builder = provider
        .send_raw_transaction(&tx)
        .await?
        .with_required_confirmations(2)
        .with_timeout(Some(std::time::Duration::from_secs(60)));
    // Register the pending transaction with the provider.
    let pending_tx = builder.register().await?;
    // Wait for the transaction to be confirmed 2 times.
    let tx_hash = pending_tx.await?;
    Ok(axum::Json(TransactionResponse {
        transaction_hash: format!("{:#x}", tx_hash),
        block_number: None,
        status: TransactionStatus::Confirmed,
    }))
}
