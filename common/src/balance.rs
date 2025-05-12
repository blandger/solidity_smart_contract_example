use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    // Wallet balance in wei
    pub balance: U256,
}
