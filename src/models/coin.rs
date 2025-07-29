use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Coin {
    pub id: String,

    pub symbol: String,
    pub name: String,
    pub image: String,

    pub current_price: BigDecimal,
    pub market_cap: Option<BigInt>,
    pub market_cap_rank: Option<i32>,
    pub fully_diluted_valuation: Option<BigInt>,
    pub total_volume: Option<BigInt>,
    pub high_24h: Option<BigInt>,
    pub low_24h: Option<BigInt>,

    pub price_change_24h: Option<BigDecimal>,
    pub price_change_percentage_24h: Option<BigDecimal>,

    pub market_cap_change_24h: Option<BigInt>,
    pub market_cap_change_percentage_24h: Option<BigDecimal>,

    pub circulating_supply: Option<BigInt>,
    pub total_supply: Option<BigInt>,
    pub max_supply: Option<BigInt>,

    pub ath: Option<BigDecimal>,
    pub ath_change_percentage: Option<BigDecimal>,
    pub ath_date: Option<NaiveDateTime>,

    pub atl: Option<BigDecimal>,
    pub atl_change_percentage: Option<BigDecimal>,
    pub atl_date: Option<NaiveDateTime>,

    pub roi: Option<ROI>,
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ROI {
    pub times: Option<BigDecimal>,
    pub currency: Option<String>,
    pub percentage: Option<BigDecimal>,
}
