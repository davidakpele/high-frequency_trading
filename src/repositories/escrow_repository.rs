use sqlx::{MySqlPool, Result};
use chrono::Utc;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use crate::enums::order_status::OrderStatus;

pub struct EscrowRepository {
    pub db: MySqlPool,
}

impl EscrowRepository {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    pub async fn create_escrow(
        &self,
        order_id: String,
        amount: BigDecimal,
        status: OrderStatus,
    ) -> Result<()> {
        let created_at = Utc::now().naive_utc();
        let id = Uuid::new_v4().to_string();
        
        sqlx::query(
            "INSERT INTO escrow (id, order_id, amount, status, created_at)
            VALUES (?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(&order_id)
        .bind(amount)
        .bind(status)
        .bind(created_at)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn update_status(&self, order_id: String, status: OrderStatus)-> Result<()>{
        sqlx::query("UPDATE escrow SET status =? WHERE order_id =?")
        .bind(status)
        .bind(order_id)
        .execute(&self.db)
        .await?;

        Ok(())  
    
    }

}
