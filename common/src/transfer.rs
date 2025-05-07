use serde::{Deserialize, Serialize};
use crate::store::TransactionStatus;

#[derive(Deserialize)]
pub struct TransferPayload {
    pub address_from: String,
    pub address_to: String,
    pub amount: f64,
    pub signed_transaction: String,
}

#[derive(Serialize)]
pub struct TransferTransactionResponse {
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
}
