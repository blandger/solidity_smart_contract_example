use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::reqwest::Url;
use common::config::init_test_net_url;
use common::error::ApiError;
use std::sync::Arc;
use axum::extract::FromRef;

pub fn create_shared_provider() -> Result<Arc<dyn Provider>, ApiError> {
    let rpc_url = init_test_net_url();
    println!("Init ETH RPC provider by url: '{}'", &rpc_url);
    let rpc_url = Url::parse(rpc_url)?;
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

impl FromRef<AppState> for Arc<dyn Provider> {
    fn from_ref(state: &AppState) -> Arc<dyn Provider> {
        state.provider.clone()
    }
}
