use serde::{Deserialize, Serialize};
use crate::store::TransactionStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployContractPayload {
    pub address_from: String,
    /// Hex-encoded signed transaction
    pub signed_transaction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployContractResponse {
    /// Contract deployed address
    pub contract_address: String,
    /// Transaction's hash
    pub transaction_hash: Option<String>,
    /// Block number with deployed contract
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
}
