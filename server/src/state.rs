use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::reqwest::Url;
use common::config::TEST_NET_RPC_URL;
use common::error::ApiError;
use std::sync::Arc;

pub fn create_shared_provider() -> Result<Arc<dyn Provider>, ApiError> {
    let rpc_url = Url::parse(TEST_NET_RPC_URL)?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    Ok(Arc::new(provider))
}

#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<dyn Provider>,
}

impl AppState {
    pub fn new(provider: Arc<dyn Provider>) -> Self {
        Self { provider }
    }
}
