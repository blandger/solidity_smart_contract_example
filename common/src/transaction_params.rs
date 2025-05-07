use serde::{Deserialize, Serialize};

// Structure for transaction parameters response from the server
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionParamsResponse {
    pub nonce: u64,
    pub gas_price: u128,
    pub chain_id: u64,
}
