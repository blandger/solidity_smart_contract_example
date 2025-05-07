use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use url::ParseError;
use alloy::hex;
use alloy::transports::{RpcError, TransportErrorKind};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Ethereum node connection error: {0}")]
    NodeConnectionError(String),

    #[error("Invalid transaction format: {0}")]
    InvalidTransactionFormat(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

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
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NodeConnectionError(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::InvalidTransactionFormat(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::InsufficientFunds(_) => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            ApiError::ContractDeploymentError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::TransactionTimeout(_) => (StatusCode::GATEWAY_TIMEOUT, self.to_string()),
            ApiError::InternalServerError(_) /*| ApiError::EthereumError(_) */=>
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::IncorrectUrl(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::IncorrectTxData(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::SendTx(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::PendingTx(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
