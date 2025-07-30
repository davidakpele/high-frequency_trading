use std::net::SocketAddr;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    models::order::Order, payloads::order_payload::CreateOrderPayload, services::trade_engine::OrderBook, ws::ws_channel::WsBroadcaster
};

pub async fn handle_ws_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    client_id: Uuid,
    peer: SocketAddr,
    broadcaster: WsBroadcaster,
    order_book: Arc<Mutex<OrderBook>>,
) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Register client
    broadcaster.add_client(client_id, tx).await;
    
    // // Send welcome message
    // let _ = ws_sender.send(Message::Text(
    //     json!({
    //         "event": "connected",
    //         "client_id": client_id,
    //         "timestamp": chrono::Utc::now().timestamp_millis()
    //     }).to_string()
    // )).await;

    // Message processing task
    let process_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => handle_text_message(
                    &text,
                    &order_book,
                    &broadcaster,
                    client_id
                ).await,
                Message::Close(_) => break,
                _ => continue,
            }
        }
        
        broadcaster.remove_client(&client_id).await;
    });

    // Message sending task
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });


    tokio::select! {
        _ = process_task => {},
        _ = send_task => {},
    }
}

async fn handle_text_message(
    text: &str,
    order_book: &Arc<Mutex<OrderBook>>,
    broadcaster: &WsBroadcaster,
    client_id: Uuid
) {
    match serde_json::from_str::<Value>(text) {
        Ok(payload) => {
            if let Some("create_order") = payload.get("type").and_then(|t| t.as_str()) {
                handle_create_order(payload, order_book, broadcaster, client_id).await;
            }
        }
        Err(e) => {
            let _ = broadcaster.send_to(
                &client_id,
                json!({
                    "status": "error",
                    "error": "Invalid JSON",
                    "details": e.to_string()
                }).to_string()
            ).await;
        }
    }
}

async fn handle_create_order(
    payload: Value,
    order_book: &Arc<Mutex<OrderBook>>,
    broadcaster: &WsBroadcaster,
    client_id: Uuid
) {
    match serde_json::from_value::<CreateOrderPayload>(payload) {
        Ok(order_data) => {
            let order = match Order::try_from(order_data) {
                Ok(order) => order,
                Err(e) => {
                    let _ = broadcaster.send_to(
                        &client_id,
                        json!({
                            "status": "error",
                            "error": "Invalid order data",
                            "details": e
                        }).to_string()
                    ).await;
                    return;
                }
            };

            let book = order_book.lock().await;
            book.add_order(order);

            let _ = broadcaster.send_to(
                &client_id,
                json!({
                    "status": "success",
                    "message": "Order added to book"
                }).to_string()
            ).await;
        }
        Err(e) => {
            let _ = broadcaster.send_to(
                &client_id,
                json!({
                    "status": "error",
                    "error": "Invalid order format",
                    "details": e.to_string()
                }).to_string()
            ).await;
        }
    }
}