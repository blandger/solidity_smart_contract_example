use common::retrieve::RetrieveMessageResponse;
use crate::config::BASE_LOCAL_SERVER_URL;
use crate::errors::ClientError;
use crate::errors::ClientError::Server;

pub async fn read_message(contact_hash: &str) -> Result<RetrieveMessageResponse, ClientError> {
    println!("Reading 'message' from contract deployed by address: {}", &contact_hash);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minutes
        .build()
        .expect("Failed to build reqwest client");

    let contract_call_response = client
        .get(format!("{}/retrieve-message/{}", &BASE_LOCAL_SERVER_URL, &contact_hash))
        .send()
        .await?;

    if !contract_call_response.status().is_success() {
        let error_text = contract_call_response.text().await?;
        println!("Failed to get message from address '{}' because: {}", &contact_hash, error_text);
        return Err(Server(error_text));
    }

    let contract_call_result = contract_call_response
        .json::<RetrieveMessageResponse>()
        .await?;
    println!("Got message from contract: '{:?}'...", &contract_call_result);
    Ok(contract_call_result)
}
