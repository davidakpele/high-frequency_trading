use dashmap::DashMap;
use std::collections::VecDeque;
use tokio::sync::mpsc;
use num::Zero;

use crate::{enums::types::OrderSide, matching::trade::Trade, models::order::Order};

#[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

                    // Borrow the quantities to avoid moving them
                    let qty = bid.quantity.clone().min(ask.quantity.clone());
                    
                    trades.push(Trade::new(bid.clone(), ask.clone(), qty.clone()));

                    // Mutate in-place
                    bids[0].quantity -= qty.clone();
                    asks[0].quantity -= qty;

                    if bids[0].quantity.is_zero() {
                        bids.pop_front();
                    }
                    if asks[0].quantity.is_zero() {
                        asks.pop_front();
                    }
                }
            }
        }
        
        trades
    }
}