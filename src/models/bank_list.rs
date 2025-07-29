use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct BankList {
    pub id: i64,
    pub user_id: i64,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
}
