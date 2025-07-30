use dashmap::DashMap;
use std::collections::VecDeque;
use tokio::sync::mpsc;

use crate::{enums::types::OrderSide, matching::trade::Trade, models::order::Order};

pub struct OrderBook {
    bids: DashMap<String, VecDeque<Order>>, 
    asks: DashMap<String, VecDeque<Order>>,
    trade_tx: mpsc::UnboundedSender<Trade>, 
}

impl OrderBook {
    pub fn new(trade_tx: mpsc::UnboundedSender<Trade>) -> Self {
        Self {
            bids: DashMap::new(),
            asks: DashMap::new(),
            trade_tx,
        }
    }

    pub fn send_trade(&self, trade: Trade) -> Result<(), mpsc::error::SendError<Trade>> {
        self.trade_tx.send(trade)
    }

    #[inline]
    pub fn add_order(&self, order: Order) {
        let book = match order.side {
            OrderSide::Buy => &self.bids,
            OrderSide::Sell => &self.asks,
        };
        
        book.entry(order.symbol.clone())
           .or_default()
           .push_back(order);
    }

    pub fn match_orders(&mut self) -> Vec<Trade> { 
        let mut trades = Vec::new();
        
        // Collect symbols first to avoid holding locks during iteration
        let symbols: Vec<String> = self.bids.iter()
            .map(|entry| entry.key().clone())
            .collect();

        for symbol in symbols {
            // Get mutable references to both sides
            if let (Some(mut bids), Some(mut asks)) = (
                self.bids.get_mut(&symbol),
                self.asks.get_mut(&symbol),
            ) {
                // Match orders while both sides have liquidity
                while !bids.is_empty() && !asks.is_empty() {
                    let (bid, ask) = (bids[0].clone(), asks[0].clone());
                    
                    if bid.price >= ask.price {
                        let qty = bid.quantity.min(ask.quantity);
                        trades.push(Trade::new(bid, ask, qty));
                        
                        // Update quantities in place
                        bids[0].quantity -= qty;
                        asks[0].quantity -= qty;
                        
                        // Remove filled orders
                        if bids[0].quantity == 0 { bids.pop_front(); }
                        if asks[0].quantity == 0 { asks.pop_front(); }
                    } else {
                        break;  // No more crosses at this price level
                    }
                }
            }
        }
        
        trades
    }
}