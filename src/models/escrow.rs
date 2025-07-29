use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::order_status::OrderStatus;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Escrow {
    pub id: i64,
    pub order_id: String,
    pub amount: BigDecimal,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
}
