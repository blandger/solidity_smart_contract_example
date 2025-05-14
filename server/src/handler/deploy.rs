use std::time::Duration;
use alloy::hex;
use axum::extract::State;
use axum::Json;
use common::deploy::{DeployContractPayload, DeployContractResponse};
use common::error::ApiError;
use common::store::TransactionStatus;
use crate::state::AppState;

pub async fn deploy_contract(
    State(state): State<AppState>,
    Json(payload): Json<DeployContractPayload>,
) -> Result<Json<DeployContractResponse>, ApiError> {
    println!("deploy_contract... address: {}", &payload.address_from);

    let provider = state.provider;

    let tx_bytes = hex::decode(&payload.signed_transaction)?;
    let tx: &[u8] = &tx_bytes;

    println!("Sending smart contract deploy tx ...");
    let builder = provider
        .send_raw_transaction(tx)
        .await?
        .with_required_confirmations(2)
        .with_timeout(Some(Duration::from_secs(60)));

    let pending_tx = builder.register().await?;

    let receipt_tx_hash = pending_tx.await?;

    println!("Sent deploy tx: '{}' ...", &receipt_tx_hash);

    let receipt = provider
        .get_transaction_receipt(receipt_tx_hash)
        .await?;

    println!("Got receipt for tx: {}", &receipt_tx_hash);

    let receipt = receipt.ok_or_else(|| ApiError::ReceiptNotFound(receipt_tx_hash))?;

    let block_number = receipt.block_number.ok_or_else(|| ApiError::ReceiptBlockNotFound(receipt_tx_hash))?;

    let contract_address = receipt.contract_address
        .ok_or_else(|| ApiError::ReceiptContractAddressNotFound(receipt_tx_hash))?;

    println!(
        "Contract deployed at address '{}' in block '{}",
        contract_address, block_number
    );

    Ok(Json(DeployContractResponse {
        transaction_hash: Some(receipt_tx_hash.to_string()),
        contract_address: contract_address.to_string(),
        block_number: Some(block_number),
        status: TransactionStatus::Confirmed,
    }))
}
