use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployContractPayload {
    /// Hex-encoded signed transaction
    signed_transaction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployContractResponse {
    /// Contract deployed address
    contract_address: String,
    /// Transaction's hash
    transaction_hash: String,
    /// Block number with deployed contract
    block_number: u64,
}
