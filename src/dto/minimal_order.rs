use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct MinimalOrder {
    pub id: String,
    pub user_id: String,
    pub is_maker: bool,
    pub trading_pair: String,
    pub price: BigDecimal,
    pub amount: BigDecimal,
    pub order_type: String,
    pub status: String,
    pub filled_amount: BigDecimal,
    pub bank_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
