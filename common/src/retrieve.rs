use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RetrieveMessageQuery {
    contract_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct RetrieveMessageResponse {
    /// Message from contract
    pub message: String,
    /// Optional: block number on last storing
    pub last_updated_block: Option<u64>,
}
