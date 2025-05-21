use crate::error::ApiError;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::transports::http::reqwest::Url;
use std::sync::Arc;

pub type DefaultEthProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

pub fn create_shared_provider() -> Result<Arc<DefaultEthProvider>, ApiError> {
    let rpc_url = Url::parse(crate::config::init_test_net_url())?;
    println!("gRPC provider url: '{}'", &rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    Ok(Arc::new(provider))
}
