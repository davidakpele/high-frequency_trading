use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR")]
pub enum OrderSide {
    BUY,
    SELL,
}