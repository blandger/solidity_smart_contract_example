use std::error::Error;

pub async fn read_message(contact_hash: &str) -> Result<(), Box<dyn Error>> {
    println!("Reading message from contract by hash: {}", &contact_hash);
    Ok(())
}
