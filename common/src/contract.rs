use crate::error::ApiError;
use crate::provider::DefaultEthProvider;
use alloy::contract::{ContractInstance, Interface};
use alloy::core::sol;
use alloy::eips::Encodable2718;
use alloy::hex;
use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolCall;
use std::sync::Arc;

// Generate contract interface from ABI
sol!(MessageStorage, "src/MessageStorage.abi");

/// Contract ABI from include_str
pub const ABI: &str = include_str!("MessageStorage.abi");

/// Contract wrapper for easier calling our contract methods
pub struct MessageStorageContract {
    /// Deployed contract address
    address: Address,
    /// Contract instance
    contract_instance: ContractInstance<Arc<DefaultEthProvider>, Ethereum>,
}

impl MessageStorageContract {
    /// Create a new contract instance for later use by client and server
    pub fn new(
        contract_address: Address,
        provider: Arc<DefaultEthProvider>,
    ) -> Result<Self, ApiError> {
        // Get the contract ABI from include_str.
        let abi = serde_json::from_str(ABI)?;

        // Create a new `ContractInstance` from the abi
        let contract =
            ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));
        Ok(Self {
            address: contract_address,
            contract_instance: contract,
        })
    }

    /// That method is used by Axum server handler
    pub async fn retrieve_message(&self) -> Result<String, ApiError> {
        println!(
            "Retrieve message by contract address: '{}'...",
            self.address
        );
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

    /// Method is used by CLI client because of signing transaction by private key
    pub async fn store_message_hex(
        &self,
        sender_wallet: PrivateKeySigner,
        new_message: &str,
    ) -> Result<String, ApiError> {
        println!("store_message_hex( {} ) ", &new_message);
        let call = MessageStorage::storeMessageCall {
            _message: new_message.to_string(),
        };
        let call_data = call.abi_encode();
        println!("encoded call data: {:?}", &call_data);

        // Contract user owner address
        let sender = sender_wallet.address();
        println!("Getting nonce for sender: {:?}", sender);
        let nonce = self.contract_instance
            .provider()
            .get_transaction_count(sender)
            .await?;
        let gas_price = self.contract_instance.provider().get_gas_price().await?;

        let mut tx_request = TransactionRequest::default()
            .with_from(sender)
            .with_to(self.address)
            .input(call_data.into())
            .with_nonce(nonce)
            .with_gas_price(gas_price);

        let gas_limit = self
            .contract_instance
            .provider()
            .estimate_gas(tx_request.clone())
            .await?;
        tx_request = tx_request.with_gas_limit(gas_limit);
        println!("Received data: nonce={:?}, gas_price={:?}, gas_limit={:?}", &nonce, &gas_price, &gas_limit);

        let wallet = EthereumWallet::from(sender_wallet);
        let tx_envelope = tx_request.build(&wallet).await?;

        let serialized_tx = tx_envelope.encoded_2718();
        let signed_tx_hex = hex::encode(serialized_tx.as_slice());
        println!("Prepared tx={:?}", &signed_tx_hex);
        // Return hex strings for sending to server
        Ok(signed_tx_hex)
    }
}
