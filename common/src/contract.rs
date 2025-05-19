use std::sync::Arc;
use crate::error::ApiError;
use alloy::contract::{ContractInstance, Interface};
use alloy::core::sol;
use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::signers::Signer;
use crate::provider::DefaultEthProvider;

// Generate contract interface from ABI
sol!(MessageStorage, "src/MessageStorage.abi");

pub const ABI: &str = include_str!("MessageStorage.abi");

pub struct MessageStorageContract {
    address: Address,
    contract_instance: ContractInstance<Arc<DefaultEthProvider>, Ethereum>,
}

impl MessageStorageContract {
    pub fn new(contract_address: Address, provider: Arc<DefaultEthProvider>,) -> Result<Self, ApiError> {
        // Get the contract ABI.
        let abi = serde_json::from_str(&ABI)?;

        // Create a new `ContractInstance` of the `Counter` contract from the abi
        let contract = ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));
        Ok(Self {
            address: contract_address,
            contract_instance: contract,
        })
    }

    pub async fn retrieve_message(&self) -> Result<String, ApiError> {
        println!("Retrieve message by contract address: '{}'...", self.address);
        let tx_builder = self.contract_instance.function("retrieveMessage", &[])?;
        let tx = tx_builder.call().await?;
        println!("contract tx = {:?}", &tx);
        let message = tx
            .first()
            .ok_or_else(|| ApiError::EmptyReadMethod("No result returned from contract".into()))?
            .as_str()
            .ok_or_else(|| ApiError::EmptyReadMethod("Result is not a string".into()))?;
        println!("Got message from contract: '{}'...", &message);
        Ok(message.to_string())
    }

    pub async fn store_message_hex(
        &self,
        wallet: &impl Signer,
        message: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        /*let call = MessageStorage::storeMessageCall { _message: message };
        let calldata = call.encode();

        let sender = wallet.address();
        let nonce = self.provider.get_transaction_count(&sender, None).await?;
        let gas_price = self.provider.get_gas_price().await?;

        let mut tx = alloy_primitives::TxRequest::new()
            .to(self.address)
            .data(calldata)
            .nonce(nonce)
            .gas_price(gas_price);

        let gas_limit = self.provider.estimate_gas(&sender, &tx, None).await?;
        tx = tx.gas(gas_limit);

        let signed_tx = wallet.sign_transaction(tx).await?;

        // Возвращаем hex строки для передачи на сервер
        Ok(format!("0x{}", hex::encode(signed_tx)))*/
        todo!()
    }
}
