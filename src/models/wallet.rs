use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: String,
    pub user_id: String,
    pub crypto_id: String,
    pub balance: BigDecimal,
    pub wallet_address: Option<String>,
    pub version: i32,
}