use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use crate::enums::{order_type::OrderType, types::OrderSide};
use crate::payloads::order_payload::CreateOrderPayload;
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub symbol: String,
    pub price: Option<BigDecimal>,  
    pub quantity: BigDecimal,
    pub timestamp: u64, 
}

impl TryFrom<CreateOrderPayload> for Order {
    type Error = String;

    fn try_from(payload: CreateOrderPayload) -> Result<Self, Self::Error> {
        let order_id: u64 = 0;

        Ok(Order {
            id: order_id,
            side: payload.side,
            order_type: payload.order_type,
            symbol: payload.symbol,
            price: payload.price,
            quantity: payload.quantity,
            timestamp: chrono::Utc::now().timestamp_nanos() as u64,
        })
    }
}
