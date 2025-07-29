use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderPayload {
    pub user_id: i64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub bank_id: Option<i64>,
}
