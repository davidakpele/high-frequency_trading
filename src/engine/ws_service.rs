use bigdecimal::BigDecimal;
use serde_json::{json, Value};
use std::str::FromStr;
use anyhow::{anyhow, Result};
use crate::{enums::order_type::OrderType, services::order_service::OrderService};

#[allow(dead_code)]
pub async fn handle_trade_command(order_service: &OrderService, payload: Value) -> Result<String> {
    let user_id = payload.get("user_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow!("Missing or invalid user_id"))?;

    let trading_pair = payload.get("trading_pair")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing or invalid trading_pair"))?;

    let order_type_str = payload.get("order_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing or invalid order_type"))?;

    let order_type = match order_type_str {
        "buy" => OrderType::BUY,
        "sell" => OrderType::SELL,
        _ => return Err(anyhow!("Invalid order_type")),
    };

    let price = payload.get("price")
        .and_then(|v| v.as_str())
        .and_then(|s| BigDecimal::from_str(s).ok())
        .ok_or_else(|| anyhow!("Missing or invalid price"))?;

    let amount = payload.get("amount")
        .and_then(|v| v.as_str())
        .and_then(|s| BigDecimal::from_str(s).ok())
        .ok_or_else(|| anyhow!("Missing or invalid amount"))?;

    let bank_id = payload.get("bank_id")
        .and_then(|v| v.as_i64());

    order_service.create_order(user_id, trading_pair.to_string(), order_type, price, amount, bank_id).await?;


    let response = json!({
        "event": "trade_confirmed",
        "status": "ok",
        "message": "Trade received and validated"
    });

    Ok(response.to_string())
}
