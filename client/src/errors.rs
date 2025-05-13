use std::path::PathBuf;
use alloy::hex::FromHexError;
use alloy::transports::TransportError;
use k256::ecdsa;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("IO Error reading key file")]
    IO(#[from] std::io::Error),
    #[error("Private key file '{0}.private' is not found by path '{1}'")]
    ReadPrivateKey(String, PathBuf),
    #[error(transparent)]
    Hex(#[from] FromHexError),
    #[error(transparent)]
    PrivateKeyFromBytes(#[from] ecdsa::Error),
    #[error(transparent)]
    Connect(#[from] TransportError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("Error from server API: '{0}")]
    Server(String),
    #[error("Contract binary file '{0}.bin' is not found by path '{1}'")]
    NotFoundContractFile(String, PathBuf),
    #[error("Contract binary file '{0}.bin' reading error by path '{1}'")]
    ReadContractFile(String, PathBuf),
}
