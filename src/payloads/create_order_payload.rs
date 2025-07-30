use serde::Deserialize;
use crate::enums::{order_type::OrderType, types::OrderSide};

#[derive(Debug, Deserialize)]
pub struct CreateOrderPayload {
    pub side: OrderSide,
    pub order_type: OrderType,
    pub symbol: String,
    pub price: Option<u64>, 
    pub quantity: u64,
}
