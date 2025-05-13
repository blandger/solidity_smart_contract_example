use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use url::ParseError;
use alloy::hex;
use alloy::primitives::{AddressError, TxHash, U256};
use alloy::transports::{RpcError, TransportErrorKind};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Ethereum node connection error: {0}")]
    NodeConnectionError(String),

    #[error("Invalid transaction format: {0}")]
    InvalidTransactionFormat(String),

    #[error("Insufficient funds on address: {0} for amount = {1} ETH")]
    InsufficientFunds(String, U256),

    #[error("Contract deployment error: {0}")]
    ContractDeploymentError(String),

    #[error("Transaction timeout after {0} seconds")]
    TransactionTimeout(u64),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    // #[error("Ethereum error: {0}")]
    // EthereumError(#[from] ethers::core::types::Error),

    #[error(transparent)]
    IncorrectUrl(#[from] ParseError),

    #[error(transparent)]
    IncorrectTxData(#[from] hex::FromHexError),
    #[error(transparent)]
    SendTx(#[from] RpcError<TransportErrorKind>),
    #[error(transparent)]
    PendingTx(#[from] alloy::providers::PendingTransactionError),
    #[error("Incorrect Address: {0}")]
    IncorrectAddress(#[from] AddressError),
    #[error("Receipt not found by tx hash: {0}")]
    ReceiptNotFound(TxHash),
    #[error("Receipt do not have 'block number' by tx hash: {0}")]
    ReceiptBlockNotFound(TxHash),
    #[error("Receipt do not have 'contract address' by tx hash: {0}")]
    ReceiptContractAddressNotFound(TxHash),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NodeConnectionError(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::InvalidTransactionFormat(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::InsufficientFunds(_, _) => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            ApiError::ContractDeploymentError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::TransactionTimeout(_) => (StatusCode::GATEWAY_TIMEOUT, self.to_string()),
            ApiError::InternalServerError(_) /*| ApiError::EthereumError(_) */=>
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::IncorrectUrl(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::IncorrectTxData(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::SendTx(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::PendingTx(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::IncorrectAddress(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ReceiptNotFound(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ReceiptBlockNotFound(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ReceiptContractAddressNotFound(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
