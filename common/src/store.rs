use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreMessagePayload {
    /// Contract's address
    pub contract_address: String,
    /// Hex-encoded signed transaction for calling storeMessage
    pub signed_transaction: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInContractTransactionResponse {
    /// Transaction hash
    pub transaction_hash: String,
    /// Block number (if transaction is stored in block)
    pub block_number: Option<u64>,
    /// Transaction status (sent, confirmed, cancelled)
    pub status: TransactionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    /// Error reason
    Failed(String),
}
