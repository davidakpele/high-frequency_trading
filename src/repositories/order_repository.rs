use crate::{dto::minimal_order::MinimalOrder, enums::order_status::OrderStatus};
use sqlx::MySqlPool;
use bigdecimal::BigDecimal;
use chrono::Utc;
use uuid::Uuid;
use crate::enums::order_type::OrderType;

pub struct OrderRepository {
    pub db: MySqlPool,
}

impl OrderRepository {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    pub async fn save_order(
        &self,
        user_id: i64,
        is_maker: bool,
        trading_pair: &str,
        order_type: OrderType,
        price: &BigDecimal,
        amount: &BigDecimal,
        filled_amount: &BigDecimal,
        status: OrderStatus,
        bank_id: Option<i64>,
    ) -> sqlx::Result<MinimalOrder> {
        let now = Utc::now().naive_utc();
        let order_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO orders (
                id, user_id, is_maker, trading_pair, order_type, price, amount,
                filled_amount, status, bank_id, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&order_id)
        .bind(user_id)
        .bind(is_maker)
        .bind(trading_pair)
        .bind(order_type)
        .bind(price)
        .bind(amount)
        .bind(filled_amount)
        .bind(status)
        .bind(bank_id)
        .bind(now)
        .bind(now)
        .execute(&self.db)
        .await?;

        let order = sqlx::query_as::<_, MinimalOrder>(
            "SELECT id, user_id, is_maker, trading_pair, price, status, order_type, amount, filled_amount, bank_id, created_at, updated_at
            FROM orders WHERE id = ?"
        )
        .bind(&order_id)
        .fetch_one(&self.db)
        .await?;

        Ok(order)
    }


    pub async fn  update_filled_amount_and_status(
        &self,
        order_id: String,
        filled_amount: &BigDecimal,
        status: &str,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE orders SET filled_amount = ?, status = ? WHERE id = ?",
            filled_amount,
            status,
            order_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }


   pub async fn find_matching_orders(
        &self,
        trading_pair: &str,
        price: &BigDecimal,
        order_type: &OrderType,
    ) -> sqlx::Result<Vec<MinimalOrder>> {
        let opposite_type = match order_type {
            OrderType::BUY => "SELL",
            OrderType::SELL => "BUY",
            _ => {
                return Ok(vec![]);
            }
        };

        let orders = sqlx::query_as::<_, MinimalOrder>(
            "SELECT * FROM orders
            WHERE trading_pair = ?
            AND order_type = ?
            AND price = ?
            AND status = 'open'
            ORDER BY created_at ASC",
        )
        .bind(trading_pair)
        .bind(opposite_type)
        .bind(price)
        .fetch_all(&self.db)
        .await?;

        Ok(orders)
    }


}
