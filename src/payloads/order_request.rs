use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::enums::order_side::OrderSide;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderRequest {
    pub user_id: i64,
    pub wallet_id: i64,
    pub symbol: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub side: OrderSide,
}
