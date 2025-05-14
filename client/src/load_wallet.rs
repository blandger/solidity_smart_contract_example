use std::fs;
use std::path::Path;
use std::str::FromStr;
use alloy::signers::local::PrivateKeySigner;

use alloy::hex;
use alloy_primitives::Address;
use crate::errors::ClientError;
use crate::get_path;

/// Load private key from file with name using current folder
pub fn load_wallet_from_file(name: &str) -> Result<PrivateKeySigner, ClientError> {
    // Read private key from file
    let private_key_path = get_path(&format!("{}.private", name));
    if !Path::new(&private_key_path).exists() {
        return Err(ClientError::ReadPrivateKey(name.to_string(), private_key_path));
    }
    // Read hex-string from file
    let hex_str = fs::read_to_string(private_key_path)?;
    // Decode hex-string into bytes
    let bytes = hex::decode(hex_str.trim())?;
    // Create PrivateKey from bytes
    let wallet = PrivateKeySigner::from_slice(&bytes)?;
    Ok(wallet)
}

/// Get recipient address by hex or by local private key file 
pub fn recipient_address_from_string_or_local_file(name: &str) -> Result<Address, ClientError> {
    // check if name is a valid RECIPIENT public address first
    if Address::from_str(name).is_ok() {
        return Ok(Address::from_str(name)?);
    }
    
    // Usually we DON'T HAVE a RECIPIENT private key,
    // but we can do that that way because testing purposes
    
    // Try to read RECIPIENT private key from file (test cases only)
    let private_key_path = get_path(&format!("{}.private", name));
    if !Path::new(&private_key_path).exists() {
        return Err(ClientError::ReadPrivateKey(name.to_string(), private_key_path));
    }
    // Read hex-string from file
    let hex_str = fs::read_to_string(private_key_path)?;
    // Decode hex-string into bytes
    let bytes = hex::decode(hex_str.trim())?;
    // Create PrivateKey from bytes
    let wallet = PrivateKeySigner::from_slice(&bytes)?;
    Ok(wallet.address())
}
