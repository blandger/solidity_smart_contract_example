pub mod handler;
pub mod state;

use crate::handler::balance::get_balance_route;
use crate::handler::transaction_params::get_transaction_params_route;
use crate::handler::transfer::transfer;
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use common::init_log::init_tracing;
use common::provider::create_shared_provider;
use handler::deploy::deploy_contract;
use handler::hello::static_hello;
use handler::read::retrieve_message_route;
use handler::store::store_message;
use std::error::Error;
use std::net::SocketAddr;
use tracing::info;

const MODULE_LOG_FILTERS: &str = concat!("server=INFO,");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing(MODULE_LOG_FILTERS);

    let state = AppState::new(create_shared_provider()?);

    let api_routes = Router::new()
        .route("/test", get(static_hello))
        .route("/balance/{address}", get_balance_route())
        .route("/transfer", post(transfer))
        .route("/deploy-contract", post(deploy_contract))
        .with_state(state.clone())
        .route("/store-message", post(store_message))
        .with_state(state.clone())
        .route(
            "/retrieve-message/{contract_address}",
            retrieve_message_route(),
        )
        .with_state(state.clone())
        .route("/tx/{address}", get_transaction_params_route())
        .with_state(state.clone());

    let app = with_prefix("/api", api_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server is listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn with_prefix(prefix: &str, routes: Router) -> Router {
    Router::new().nest(prefix, routes)
}
