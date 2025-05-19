use std::sync::Arc;
use common::provider::DefaultEthProvider;

#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<DefaultEthProvider>,
}

impl AppState {
    pub fn new(provider: Arc<DefaultEthProvider>) -> Self {
        Self { provider }
    }
}
