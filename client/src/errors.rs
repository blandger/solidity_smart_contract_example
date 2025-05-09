use alloy::hex::FromHexError;
use alloy::transports::TransportError;
use k256::ecdsa;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("IO Error reading key file")]
    IO(#[from] std::io::Error),
    #[error("Private key file '{0}.private' not found")]
    ReadPrivateKey(String),
    #[error(transparent)]
    Hex(#[from] FromHexError),
    #[error(transparent)]
    PrivateKeyFromBytes(#[from] ecdsa::Error),
    #[error(transparent)]
    Connect(#[from] TransportError),
}
