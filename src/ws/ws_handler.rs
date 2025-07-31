use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;
use redis::aio::Connection;
use redis::{Client, RedisError};
use anyhow::Result;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

use crate::models::asset_wallet::generate_user_assets;
use crate::{
    models::{order::Order},
    payloads::order_payload::CreateOrderPayload,
    services::trade_engine::OrderBook,
    utils::jwt::Claims,
    ws::{ws_auth::WsAuth, ws_channel::WsBroadcaster},
};

pub async fn handle_ws_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    client_id: Uuid,
    peer: SocketAddr,
    broadcaster: WsBroadcaster,
    order_book: Arc<Mutex<OrderBook>>,
    redis_client: Arc<redis::Client>,
) {
    let _ = peer;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (_tx, _rx): (UnboundedSender<String>, UnboundedReceiver<String>) = unbounded_channel();

    // Auth Stage
    let (user_id, claims) = match ws_receiver.next().await {
        Some(Ok(first_msg)) => match WsAuth::from_first_message(&first_msg).await {
            Ok(WsAuth(claims)) => (claims.sub, claims),
            Err((code, msg)) => {
                let _ = ws_sender
                    .send(Message::Text(json!({ "error": msg, "code": code.as_u16() }).to_string().into()))
                    .await;
                return;
            }
        },
        _ => {
            let _ = ws_sender
                .send(Message::Text(json!({ "error": "Missing auth message", "code": 400 }).to_string().into()))
                .await;
            return;
        }
    };

    // Cache user data and wallet
    let user_id = user_id as u64;
    if let Err(e) = cache_user_data(&redis_client, client_id, user_id, &claims).await {
        let _ = ws_sender
            .send(Message::Text(
                json!({
                    "error": format!("Cache failed: {}", e),
                    "code": 500
                })
                .to_string()
                .into(),
            ))
            .await;
        return;
    }

    // Register client
    let (tx, mut rx) = mpsc::unbounded_channel();
    broadcaster.add_client(client_id, tx).await;

    // Send latest data from Redis
    if let Ok(initial_data) = get_initial_data_from_redis(&redis_client).await {
        let _ = broadcaster
            .send_to(
                &client_id,
                json!({
                    "event": "initial_state",
                    "data": initial_data
                })
                .to_string(),
            )
            .await;
    }

    // Clone for async tasks
    let order_book = order_book.clone();
    let broadcaster = broadcaster.clone();
    let redis_client = redis_client.clone();

    // Message processing task
    let process_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    handle_text_message(
                        &text,
                        &order_book,
                        &broadcaster,
                        &redis_client,
                        client_id,
                    )
                    .await;
                }
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

async fn cache_user_data(
    redis: &Client,
    client_id: Uuid,
    user_id: u64,
    claims: &Claims,
) -> Result<(), RedisError> {
    let mut conn: Connection = redis.get_async_connection().await?;

    let claims_json = serde_json::to_string(claims).map_err(|e| {
        RedisError::from((
            redis::ErrorKind::TypeError,
            "Failed to serialize claims",
            e.to_string(),
        ))
    })?;

    // Generate or retrieve wallet from Redis
    let wallet_key = format!("wallet:{}", user_id);
    let exists: bool = redis::cmd("EXISTS")
        .arg(&wallet_key)
        .query_async(&mut conn)
        .await?;

    if !exists {
        let wallets = generate_user_assets(&user_id.to_string());
        let serialized = serde_json::to_string(&wallets).unwrap_or("{}".to_string());
        redis::cmd("SET")
            .arg(&wallet_key)
            .arg(serialized)
            .query_async::<_, ()>(&mut conn)
            .await?;
    }

    redis::pipe()
        .atomic()
        .hset("user:data", user_id, claims_json)
        .hset("client:map", client_id.to_string(), user_id)
        .expire("user:data", 3600)
        .query_async::<_, ()>(&mut conn)
        .await?;

    Ok(())
}

async fn handle_text_message(
    text: &str,
    order_book: &Arc<Mutex<OrderBook>>,
    broadcaster: &WsBroadcaster,
    redis_client: &Arc<Client>,
    client_id: Uuid,
) {
    match serde_json::from_str::<Value>(text) {
        Ok(payload) => {
            if let Some("create_order") = payload.get("type").and_then(|t| t.as_str()) {
                handle_create_order(payload, order_book, broadcaster, redis_client, client_id).await;
            }
        }
        Err(e) => {
            let _ = broadcaster
                .send_to(
                    &client_id,
                    json!({
                        "status": "error",
                        "error": "Invalid JSON",
                        "details": e.to_string()
                    })
                    .to_string(),
                )
                .await;
        }
    }
}

async fn handle_create_order(
    payload: Value,
    order_book: &Arc<Mutex<OrderBook>>,
    broadcaster: &WsBroadcaster,
    redis_client: &Arc<Client>,
    client_id: Uuid,
) {
    match serde_json::from_value::<CreateOrderPayload>(payload) {
        Ok(order_data) => {
            let order = match Order::try_from(order_data) {
                Ok(order) => order,
                Err(e) => {
                    let _ = broadcaster
                        .send_to(
                            &client_id,
                            json!({
                                "status": "error",
                                "error": "Invalid order data",
                                "details": e
                            })
                            .to_string(),
                        )
                        .await;
                    return;
                }
            };

            let book = order_book.lock().await;
            book.add_order(order.clone());

            let order_json = match serde_json::to_string(&order) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("❌ Failed to serialize order: {}", e);
                    return;
                }
            };

            let mut conn = match redis_client.get_async_connection().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("❌ Redis connection failed: {}", e);
                    return;
                }
            };

            let order_key = format!("order:{}:{}", order.symbol, order.id);
            if let Err(e) = redis::cmd("SET")
                .arg(&order_key)
                .arg(&order_json)
                .query_async::<_, ()>(&mut conn)
                .await
            {
                eprintln!("❌ Redis set failed: {}", e);
                return;
            }

            let _ = broadcaster
                .send_to(
                    &client_id,
                    json!({
                        "status": "success",
                        "message": "Order added to book"
                    })
                    .to_string(),
                )
                .await;

            if let Ok(initial_data) = get_initial_data_from_redis(redis_client).await {
                let _ = broadcaster
                    .send_to(
                        &client_id,
                        json!({
                            "event": "initial_state",
                            "data": initial_data
                        })
                        .to_string(),
                    )
                    .await;
            }
        }
        Err(e) => {
            let _ = broadcaster
                .send_to(
                    &client_id,
                    json!({
                        "status": "error",
                        "error": "Invalid order format",
                        "details": e.to_string()
                    })
                    .to_string(),
                )
                .await;
        }
    }
}

async fn get_initial_data_from_redis(redis: &Client) -> Result<Value, RedisError> {
    let mut conn = redis.get_async_connection().await?;

    let keys: Vec<String> = redis::cmd("KEYS")
        .arg("order:*")
        .query_async(&mut conn)
        .await?;

    let mut orders = vec![];
    for key in keys {
        if let Ok(json_str) = redis::cmd("GET").arg(&key).query_async::<_, String>(&mut conn).await {
            if let Ok(value) = serde_json::from_str::<Value>(&json_str) {
                orders.push(value);
            }
        }
    }

    Ok(json!(orders))
}
