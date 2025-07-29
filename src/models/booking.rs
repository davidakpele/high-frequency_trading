use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};


#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Booking {
    pub id: String,
    pub order_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub created_at: Option<NaiveDateTime>,
}
