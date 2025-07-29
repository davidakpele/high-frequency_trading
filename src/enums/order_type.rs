use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "UPPERCASE")]
pub enum OrderType {
    LIMIT,
    MARKET,
    STOPLOSS,
    BUY,
    SELL,
}


impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderType::LIMIT => "LIMIT",
            OrderType::MARKET => "MARKET",
            OrderType::STOPLOSS => "STOPLOSS",
            OrderType::BUY => "BUY",
            OrderType::SELL => "SELL",
        };
        write!(f, "{}", s)
    }
}
