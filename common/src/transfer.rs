use serde::{Deserialize, Serialize};
use crate::store::TransactionStatus;

#[derive(Deserialize)]
pub struct TransferPayload {
    pub signed_transaction: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
}