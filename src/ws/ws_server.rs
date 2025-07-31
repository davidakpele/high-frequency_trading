use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use tokio::sync::Mutex;
use redis::Client;

use crate::{
    services::trade_engine::OrderBook,
    ws::{ws_channel::WsBroadcaster, ws_handler::handle_ws_connection},
};

pub async fn start_ws_server(
    addr: &str,
    broadcaster: WsBroadcaster,
    order_book: Arc<Mutex<OrderBook>>,
    redis_client: Arc<Client>,
) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind WebSocket port");

    println!("🔌 WebSocket server running at ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 0)));

        let broadcaster = broadcaster.clone();
        let order_book = order_book.clone();
        let redis_client = redis_client.clone();

        tokio::spawn(async move {
            handle_connection(stream, peer, broadcaster, order_book, redis_client).await;
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    peer: SocketAddr,
    broadcaster: WsBroadcaster,
    order_book: Arc<Mutex<OrderBook>>,
    redis_client: Arc<Client>,
) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("🌐 New connection from: {}", peer);
        let client_id = Uuid::new_v4();
        handle_ws_connection(ws_stream, client_id, peer, broadcaster, order_book, redis_client).await;
    }
}
