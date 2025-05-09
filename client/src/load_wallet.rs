use std::fs;
use std::path::Path;
use alloy::signers::local::PrivateKeySigner;

use alloy::hex;
use crate::errors::ClientError;
use crate::get_path;

/// Load private key from file with name using current folder
pub fn load_wallet_from_file(name: &str) -> Result<PrivateKeySigner, ClientError> {
    // Read private key from file
    let private_key_path = get_path(&format!("{}.private", name));
    if !Path::new(&private_key_path).exists() {
        return Err(ClientError::ReadPrivateKey(name.to_string()));
    }
    // Read hex-string from file
    let hex_str = fs::read_to_string(private_key_path)?;
    // Decode hex-string into bytes
    let bytes = hex::decode(hex_str.trim())?;
    // Create PrivateKey from bytes
    let wallet = PrivateKeySigner::from_slice(&bytes)?;
    Ok(wallet)
}
