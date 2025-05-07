use std::error::Error;
use std::path::Path;

/// Store new value in the previously deployed contract
pub async fn store_message(signer: &str, new_string: &str) -> Result<(), Box<dyn Error>> {
    println!("Storing '{}' in contract with signer key: {}", new_string, signer);

    // Read private key from file
    let private_key_path = format!("{}.private", signer);
    if !Path::new(&private_key_path).exists() {
        return Err(format!("Private key file {} not found", private_key_path).into());
    }

    let private_key = std::fs::read_to_string(&private_key_path)?;

    // Here will be your code for signing the transaction and sending to REST API
    // 1. Create an HTTP client
    let _client = reqwest::Client::new();

    // 2. Create and sign transaction with new value (placeholder)
    let _transaction_data = serde_json::json!({
        "value": new_string,
        "signature": format!("signed_with_key_{}", private_key)
    });

    // 3. Send transaction to local server
    // let response = client.post("http://localhost:8000/api/store")
    //     .json(&transaction_data)
    //     .send()
    //     .await?;

    println!("Value '{}' successfully stored in contract", new_string);

    Ok(())
}