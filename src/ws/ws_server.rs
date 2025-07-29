use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use uuid::Uuid;

use crate::ws::ws_channel::WsBroadcaster;
use crate::ws::ws_handler::handle_ws_connection;
use crate::services::order_service::OrderService;

pub async fn start_ws_server(
    addr: &str,
    broadcaster: WsBroadcaster,
    order_service: Arc<OrderService>,
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
        let order_service = order_service.clone();

        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                println!("🌐 New WebSocket connection: {}", peer);
                let client_id = Uuid::new_v4();
                handle_ws_connection(ws_stream, client_id, peer, broadcaster, order_service).await;
            }
        });
    }
}
