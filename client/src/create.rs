use std::fs;
use alloy::signers::local::PrivateKeySigner;
use k256::ecdsa::SigningKey;
use k256::elliptic_curve::rand_core::OsRng;
use crate::errors::ClientError;
use crate::get_path;

/// Create a new wallet by generating private and public keys. Write them to files with the specified name
pub fn create_wallet(name: &str) -> Result<(), ClientError> {
    println!("Creating wallet with name: {}", name);
    let mut rng = OsRng;
    let wallet = PrivateKeySigner
    ::random_with(&mut rng);
    let address = wallet.address();
    let private_key_bytes = wallet.to_bytes();
    println!("Wallet Address: {:?}", &address);

    let signing_key = SigningKey::from_slice(&private_key_bytes.0).expect("Invalid private key");

    // Getting public key
    let verifying_key = signing_key.verifying_key();
    let public_key = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key.as_bytes();

    // Here will be your code for generating private and public keys and writing them to files {name}.private and {name}.public
    let private_key_hex = alloy::hex::encode(private_key_bytes);
    println!("Private key: 0x{}", private_key_hex);
    let public_key_hex = alloy::hex::encode(public_key_bytes);
    println!("Public key: 0x{}", public_key_hex);

    // Example structure:
    let private_key_path = get_path(&format!("{}.private", name));
    let public_key_path = get_path(&format!("{}.public", name));

    // Create parent directories if they don't exist
    if let Some(parent) = private_key_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = public_key_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write keys to files
    fs::write(&private_key_path, private_key_hex)?;
    let content = format!("#{}\n{}", address, public_key_hex);
    fs::write(&public_key_path, content)?;

    println!("Wallet successfully created.\nPrivate key saved in '{}', public key in '{}'",
             private_key_path.display(), public_key_path.display());
    Ok(())
}
