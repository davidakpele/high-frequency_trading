use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "status")]
#[sqlx(rename_all = "UPPERCASE")]  
pub enum OrderStatus {
    OPEN,
    PARTIALLYFILLED,
    FILLED,
    CANCELED,
    PENDING,
    COMPLETED,
    DISPUTED,
}
