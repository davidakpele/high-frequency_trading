use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use sqlx::MySqlPool;
use crate::{
    matching::trade::Trade, repositories::trade_repository::TradeRepository, services::trade_engine::OrderBook
};

pub struct MatchingService {
    order_book: Arc<Mutex<OrderBook>>,
    trade_repo: TradeRepository,
    db_pool: MySqlPool,
}

impl MatchingService {
    pub fn new(
        db_pool: MySqlPool,
        order_book: Arc<Mutex<OrderBook>>,
        trade_repo: TradeRepository
    ) -> Self {
        Self {
            order_book,
            trade_repo,
            db_pool,
        }
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        
        loop {
            interval.tick().await;
            self.execute_matching().await;
        }
    }

    async fn execute_matching(&self) -> usize {
        let mut order_book = self.order_book.lock().await;
        let trades = order_book.match_orders();
        let trade_count = trades.len();
        
        for trade in trades {
            if let Err(e) = self.trade_repo.bulk_insert(&[trade]).await {
                eprintln!("Failed to persist trade: {}", e);
            }
        }
        
        trade_count
    }
}