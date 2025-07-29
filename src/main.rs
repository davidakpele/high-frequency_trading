use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

mod config;
mod connection;
mod controllers;
mod services;
mod repositories;
mod models;
mod utils;
mod router;
mod enums;
mod responses;
mod payloads;
mod middleware;
mod engine;
mod dto;
mod ws;

use config::settings::Settings;
use connection::db::establish_connection;
use services::order_service::OrderService;
use crate::ws::{ws_channel::WsBroadcaster, ws_server::start_ws_server};

#[tokio::main]
async fn main() {
    let _settings = Settings::new();

    let pool = match establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    let broadcaster = WsBroadcaster::new();
    let order_service = Arc::new(OrderService::new(pool.clone()));

    tokio::spawn(start_ws_server("0.0.0.0:9001", broadcaster.clone(), order_service.clone()));

    let app = router::url::create_routes(pool, broadcaster.clone())
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any));

    let listener = TcpListener::bind("0.0.0.0:8055").await.unwrap();
    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
