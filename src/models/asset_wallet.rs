use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetWallet {
    pub user_id: String,
    pub asset_id: String,            // Unique asset wallet ID (like a crypto address)
    pub symbol: String,              // "BTC", "ETH", etc.
    pub quantity: BigDecimal,        // Total quantity owned
    pub amount: BigDecimal,          // Total amount in USD or base currency
    pub filled_amount: BigDecimal,   // Amount filled through transactions
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn generate_user_assets(user_id: &str) -> HashMap<String, AssetWallet> {
    let symbols = vec!["BTC", "ETH", "BNB", "SOL", "ADA"];
    let mut assets = HashMap::new();
    let now = chrono::Utc::now().naive_utc();

    for symbol in symbols {
        let asset_wallet = AssetWallet {
            user_id: user_id.to_string(),
            asset_id: format!("{}-{}", symbol, Uuid::new_v4()),
            symbol: symbol.to_string(),
            quantity: BigDecimal::from(0),
            amount: BigDecimal::from(0),
            filled_amount: BigDecimal::from(0),
            created_at: now,
            updated_at: now,
        };
        assets.insert(symbol.to_string(), asset_wallet);
    }

    assets
}
