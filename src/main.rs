use std::net::SocketAddr;

use axum::{
    handler::Handler,
    routing::{get, post},
    Router,
};

mod cli;
mod handlers;
mod models;
mod solution;
use handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse args
    let args = cli::parse();

    // routes
    let app = Router::new()
        .route("/health", get(check_health))
        .route("/solution", post(check_solution));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404.into_service());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr: SocketAddr = args.addr.parse().expect("Unable to parse socket address");
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
