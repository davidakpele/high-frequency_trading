use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::enums::order_status::OrderStatus;
use crate::enums::order_type::OrderType;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Orders {
    pub id: String,
    pub user_id: String,
    pub is_maker: bool,
    pub trading_pair: String, // e.g., "BTC/USD"
    pub order_type: OrderType,
    pub price: BigDecimal,
    pub amount: BigDecimal,
    pub filled_amount: BigDecimal,
    pub status: OrderStatus,
    pub bank_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}