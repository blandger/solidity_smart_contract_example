use crate::state::AppState;
use alloy::hex;
use alloy::providers::Provider;
use axum::extract::State;
use tracing::{debug, info};
use common::error::ApiError;
use common::store::TransactionStatus;
use common::transfer::{TransferTransactionResponse, TransferPayload};

pub async fn transfer(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<TransferPayload>,
) -> Result<axum::Json<TransferTransactionResponse>, ApiError> {
    info!("transfer: {:?}", &payload);
    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    // Send a transaction, and configure the pending transaction.
    let builder = provider
        .send_raw_transaction(tx)
        .await?
        .with_required_confirmations(2)
        .with_timeout(Some(std::time::Duration::from_secs(60)));
    debug!("Transfer is waiting for pending tx to transfer to: {:?}", &payload.address_to);
    // Register the pending transaction with the provider.
    let pending_tx = builder.register().await?;
    // Wait for the transaction to be confirmed 2 times.
    let tx_hash = pending_tx.await?;
    info!("Transfer is DONE to transfer to: {:?}, tx_has = {}", &payload.address_to, &tx_hash);
    Ok(axum::Json(TransferTransactionResponse {
        transaction_hash: Some(tx_hash.to_string()),
        block_number: None,
        status: TransactionStatus::Confirmed,
    }))
}
