use std::sync::OnceLock;

/// Sepolia Infura test net RPC URL with DEVELOPER KEY is BETTER !
pub const PUBLIC_TEST_NET_RPC_URL: &str = "https://rpc-sepolia.rockx.com";
// INFURA_RPC_URL=https://sepolia.infura.io/v3/YOUR_PERSONAL_INFURA_KEY

#[allow(dead_code)]
static INIT_RPC_URL: OnceLock<String> = OnceLock::new();
#[allow(dead_code)]
pub fn init_test_net_url() -> &'static str {
    INIT_RPC_URL.get_or_init(|| {
        // Try to get URL from env. variable
        std::env::var("INFURA_RPC_URL")
            .unwrap_or_else(|_| PUBLIC_TEST_NET_RPC_URL.to_string())
    })
        .as_str()
}
