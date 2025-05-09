use serde::{Deserialize, Serialize};
use crate::store::TransactionStatus;

#[derive(Deserialize, Serialize, Debug)]
pub struct TransferPayload {
    pub address_from: String,
    pub address_to: String,
    pub amount: f64,
    pub signed_transaction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferTransactionResponse {
    pub transaction_hash: Option<String>,
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
}
