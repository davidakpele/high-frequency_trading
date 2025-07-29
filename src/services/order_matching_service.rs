use std::collections::{BTreeMap, VecDeque};

use crate::{
    dto::minimal_order::MinimalOrder, enums::{order_status::OrderStatus, order_type::OrderType}, models::order::Orders, repositories::{booking_repository::BookingRepository, escrow_repository::EscrowRepository, order_repository::OrderRepository}
};
use anyhow::{Result};
use bigdecimal::BigDecimal;
use sqlx::MySqlPool;

#[allow(dead_code)]
pub struct OrderMatchingService {
    pool: MySqlPool,
    repo: OrderRepository,
    book_repo: BookingRepository,
    escrow_repo: EscrowRepository,
}

impl OrderMatchingService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            repo: OrderRepository::new(pool.clone()),
            book_repo: BookingRepository::new(pool.clone()),
            escrow_repo: EscrowRepository::new(pool.clone()),
            pool,
        }
    }

    pub async fn match_orders(&self, mut new_order: MinimalOrder) -> Result<()> {
        let new_order_type:OrderType = match  new_order.order_type.to_uppercase().as_str(){
            "BUY"=> OrderType::BUY,
            "SELL"=>OrderType::SELL,
            _ => return Err(anyhow::anyhow!("Invalid Order Type: {}", new_order.order_type).into()),
        };
        let matching_orders = self
            .repo
            .find_matching_orders(&new_order.trading_pair, &new_order.price, &new_order_type)
            .await?;

        for mut matching_order in matching_orders {
            // Skip self-matching
            if new_order.user_id == matching_order.user_id {
                continue;
            }
            let remaining_new = &new_order.amount - &new_order.filled_amount;
            let remaining_match = &matching_order.amount - &matching_order.filled_amount;

            let matched_amount = remaining_new.min(remaining_match);

            if matched_amount <= BigDecimal::from(0) {
                continue;
            }

            // Update filled amounts
            new_order.filled_amount += &matched_amount;
            matching_order.filled_amount += &matched_amount;

            // Determine status
            let new_status = Self::determine_status(&new_order);
            let match_status = Self::determine_status(&matching_order);

            // Update orders
            self.repo.update_filled_amount_and_status(new_order.id.clone(), &new_order.filled_amount, &new_status).await?;
            self.repo.update_filled_amount_and_status(matching_order.id.clone(), &matching_order.filled_amount, &match_status).await?;

            // Update Escrow seller order status to pending
            let (buyer_id, seller_id) = match new_order_type {
                OrderType::BUY => (&new_order.user_id,  &matching_order.user_id),
                _ => (
                    &matching_order.user_id,
                    &new_order.user_id
                ),
            };

            // Save coin booking
            self.book_repo.save_coin_booking(
                new_order.id.to_string(), 
                buyer_id.to_string(), 
                seller_id.to_string()  
            ).await?;

            self.escrow_repo.update_status(matching_order.id, OrderStatus::PENDING).await?;
            
            // If wish to create history, create model class, service and repo and below here you call create_history,
            
            // Stop if order fully filled
            if new_status == "FILLED" {
                break;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn determine_status(order: &MinimalOrder) -> String {
        let remaining = &order.amount - &order.filled_amount;
        if remaining <= BigDecimal::from(0) {
            "PENDING".to_string()
        } else if order.filled_amount > BigDecimal::from(0) {
            "PARTIALLY_FILLED".to_string()
        } else {
            "OPEN".to_string()
        }
    }
}
