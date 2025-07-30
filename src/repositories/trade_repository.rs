use sqlx::MySqlPool;
use crate::matching::trade::Trade;
use anyhow::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TradeRepository {
    pool: MySqlPool,
}

impl TradeRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn bulk_insert(&self, trades: &[Trade]) -> Result<u64> {
        let start_time = Instant::now();
        
        if trades.is_empty() {
            return Ok(0);
        }

        let query = r#"
            INSERT INTO trades (
                bid_order_id, 
                ask_order_id, 
                symbol, 
                price, 
                quantity, 
                executed_at
            ) VALUES (?, ?, ?, ?, ?, ?)
        "#;

        let mut tx = self.pool.begin().await?;
        let mut affected_rows = 0;

        for trade in trades {
            affected_rows += sqlx::query(query)
                .bind(trade.bid_id)
                .bind(trade.ask_id)
                .bind(&trade.symbol)
                .bind(trade.price)
                .bind(trade.quantity)
                .bind(trade.timestamp)
                .execute(&mut *tx)
                .await?
                .rows_affected();
        }

        tx.commit().await?;


        Ok(affected_rows)
    }

    pub async fn bulk_insert_batched(&self, trades: Vec<Trade>, batch_size: usize) -> Result<u64> {
        let mut total_inserted = 0;
        
        for chunk in trades.chunks(batch_size) {
            total_inserted += self.bulk_insert(chunk).await?;
        }

        Ok(total_inserted)
    }
}