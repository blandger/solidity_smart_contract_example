use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StoreMessagePayload {
    /// Contract's address
    contract_address: String,
    /// Hex-encoded signed transaction for calling storeMessage
    signed_transaction: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    /// Transaction hash
    transaction_hash: String,
    /// Block number (if transaction is stored in block)
    block_number: Option<u64>,
    /// Transaction status (sent, confirmed, cancelled)
    status: TransactionStatus,
}

#[derive(Serialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    /// Error reason
    Failed(String),
}
