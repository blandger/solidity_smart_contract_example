use common::provider::DefaultEthProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<DefaultEthProvider>,
}

impl AppState {
    pub fn new(provider: Arc<DefaultEthProvider>) -> Self {
        Self { provider }
    }
}
