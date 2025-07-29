use std::net::SocketAddr;
use serde_json::{json, Value};
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use uuid::Uuid;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use crate::payloads::order_payload::CreateOrderPayload;
use crate::ws::ws_channel::WsBroadcaster;
use crate::services::order_service::OrderService;
use crate::enums::order_type::OrderType;

pub async fn handle_ws_connection(
    ws_stream: WebSocketStream<TcpStream>,
    client_id: Uuid,
    peer: SocketAddr,
    broadcaster: WsBroadcaster,
    order_service: Arc<OrderService>,
) {
    let _ = peer;
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = unbounded_channel::<String>();

    let broadcaster_tx = broadcaster.clone();
    let broadcaster_rx = broadcaster.clone();

    broadcaster_tx.add_client(client_id, tx).await;

    let welcome_msg = json!({
        "event": "connected",
        "client_id": client_id.to_string(),
        "message": "WebSocket connection established"
    });
    broadcaster_tx.send_to(&client_id, welcome_msg.to_string()).await;

    let tx_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
        broadcaster_tx.remove_client(&client_id).await;
    });


    let rx_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => {
                    match serde_json::from_str::<Value>(&text) {
                        Ok(payload) => {
                            match payload.get("type").and_then(|t| t.as_str()) {
                                Some("trade") => {
                                    match serde_json::from_value::<CreateOrderPayload>(payload.clone()) {
                                        Ok(order_data) => {
                                            let order_type = match order_data.side.to_lowercase().as_str() {
                                                "buy" => OrderType::BUY,
                                                "sell" => OrderType::SELL,
                                                _ => {
                                                    let err = json!({
                                                        "status": "error",
                                                        "client_id": client_id.to_string(),
                                                        "error": "Invalid order type",
                                                        "details": "side must be 'buy' or 'sell'"
                                                    });
                                                    broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                                    continue;
                                                }
                                            };

                                            let price = match order_data.price.parse::<BigDecimal>() {
                                                Ok(p) => p,
                                                Err(_) => {
                                                    let err = json!({
                                                        "status": "error",
                                                        "client_id": client_id.to_string(),
                                                        "error": "Invalid price format"
                                                    });
                                                    broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                                    continue;
                                                }
                                            };

                                            let amount = match order_data.amount.parse::<BigDecimal>() {
                                                Ok(a) => a,
                                                Err(_) => {
                                                    let err = json!({
                                                        "status": "error",
                                                        "client_id": client_id.to_string(),
                                                        "error": "Invalid amount format"
                                                    });
                                                    broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                                    continue;
                                                }
                                            };

                                            match order_service.create_order(
                                                order_data.user_id,
                                                order_data.symbol,
                                                order_type,
                                                price,
                                                amount,
                                                order_data.bank_id,
                                            ).await {
                                                Ok(()) => {
                                                    let res = json!({
                                                        "status": "success",
                                                        "client_id": client_id.to_string(),
                                                        "message": "Order created"
                                                    });
                                                    broadcaster_rx.send_to(&client_id, res.to_string()).await;
                                                }
                                                Err(e) => {
                                                    let err = json!({
                                                        "status": "error",
                                                        "client_id": client_id.to_string(),
                                                        "error": "Order creation failed",
                                                        "details": e.to_string()
                                                    });
                                                    broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            let err = json!({
                                                "status": "error",
                                                "client_id": client_id.to_string(),
                                                "error": "Invalid payload structure",
                                                "details": e.to_string()
                                            });
                                            broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                        }
                                    }
                                }
                                Some("match") => {
                                    let msg = json!({
                                        "status": "success",
                                        "client_id": client_id.to_string(),
                                        "message": "Match handler not implemented"
                                    });
                                    broadcaster_rx.send_to(&client_id, msg.to_string()).await;
                                }
                                _ => {
                                    let err = json!({
                                        "status": "error",
                                        "client_id": client_id.to_string(),
                                        "error": "Invalid message type. Expected 'trade' or 'match'."
                                    });
                                    broadcaster_rx.send_to(&client_id, err.to_string()).await;
                                }
                            }
                        }
                        Err(err) => {
                            let error_msg = json!({
                                "status": "error",
                                "client_id": client_id.to_string(),
                                "error": "Malformed JSON",
                                "details": err.to_string()
                            });
                            broadcaster_rx.send_to(&client_id, error_msg.to_string()).await;
                        }
                    }
                }
                Message::Close(_) => {
                    let disconnect_msg = json!({
                        "event": "disconnected",
                        "client_id": client_id.to_string(),
                        "message": "Client disconnected"
                    });
                    broadcaster_rx.send_to(&client_id, disconnect_msg.to_string()).await;
                    break;
                }
                _ => {
                    let unsupported_msg = json!({
                        "status": "error",
                        "client_id": client_id.to_string(),
                        "error": "Unsupported WebSocket message type"
                    });
                    broadcaster_rx.send_to(&client_id, unsupported_msg.to_string()).await;
                }
            }
        }
        broadcaster_rx.remove_client(&client_id).await;
    });

    tokio::select! {
        _ = tx_task => {},
        _ = rx_task => {},
    }
}
