use std::sync::Arc;
use tokio::sync::Mutex;
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
mod matching;
mod dto;
mod ws;

use config::settings::Settings;
use connection::db::establish_connection;
use crate::{
    services::{order_matching_service::MatchingService, trade_engine::OrderBook},
    ws::{ws_channel::WsBroadcaster, ws_server::start_ws_server},
};
#[tokio::main]
async fn main() {
    let _settings = Settings::new();

    let pool = match establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Setup Redis client
    let redis_client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => Arc::new(client),
        Err(e) => {
            eprintln!("❌ Failed to connect to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let (trade_tx, _trade_rx) = tokio::sync::mpsc::unbounded_channel();
    let order_book = Arc::new(Mutex::new(OrderBook::new(trade_tx)));
    let broadcaster = WsBroadcaster::new();
    let trade_repo = crate::repositories::trade_repository::TradeRepository::new(pool.clone());

    let matching_service = MatchingService::new(
        pool.clone(),
        order_book.clone(),
        trade_repo.clone()
    );

    tokio::spawn(start_ws_server(
        "0.0.0.0:9001",
        broadcaster.clone(),
        order_book.clone(),
        redis_client.clone(),
    ));

    let app = router::url::create_routes(pool, broadcaster.clone())
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any));

    let listener = TcpListener::bind("0.0.0.0:8055").await.unwrap();
    println!("🚀 Server started on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
