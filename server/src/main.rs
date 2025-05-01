mod hello;
mod deploy;
mod store;
mod retrieve;

use std::error::Error;
use std::net::SocketAddr;
use axum::routing::{get, post};
use axum::Router;
use crate::deploy::deploy_contract;
use crate::hello::static_hello;
use crate::retrieve::retrieve_message_route;
use crate::store::store_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_routes = Router::new()
        .route("/test", get(static_hello))
        .route("/deploy-contract", post(deploy_contract))
        .route("/store-message", post(store_message))
        .route("/retrieve-message/:contract_address", retrieve_message_route());

    let app = with_prefix("/api", api_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn with_prefix(prefix: &str, routes: Router) -> Router {
    Router::new().nest(prefix, routes)
}
